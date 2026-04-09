use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::sync::{RwLock, Semaphore};
use uuid::Uuid;

// ==============================
// 通用工具函数
// ==============================
pub fn utc_iso_now() -> String {
    Utc::now().to_rfc3339()
}

// ==============================
// 核心数据结构
// ==============================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: Option<i64>,
    pub uri: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSearchHit {
    pub uri: String,
    pub memory_id: Option<i64>,
    pub snippet: String,
    pub updated_at: String,
    pub priority: Option<i32>,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct GuardDecisionEvent {
    pub timestamp: String,
    pub operation: String,
    pub action: String,
    pub method: String,
    pub reason: String,
    pub target_id: Option<i64>,
    pub blocked: bool,
    pub degraded: bool,
    pub degrade_reasons: Vec<String>,
}

// ==============================
// 1. 会话搜索缓存
// ==============================
pub struct SessionSearchCache {
    hits: Arc<RwLock<HashMap<String, VecDeque<SessionSearchHit>>>>,
    session_last_seen: Arc<RwLock<HashMap<String, (f64, u64)>>>,
    touch_sequence: Arc<Mutex<u64>>,
}

impl SessionSearchCache {
    pub fn new() -> Self {
        Self {
            hits: Arc::new(RwLock::new(HashMap::new())),
            session_last_seen: Arc::new(RwLock::new(HashMap::new())),
            touch_sequence: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn record_hit(
        &self,
        session_id: Option<String>,
        uri: String,
        memory_id: Option<i64>,
        snippet: String,
        priority: Option<i32>,
        source: String,
        updated_at: Option<String>,
    ) {
        let sid = session_id.unwrap_or_else(|| "default".to_string());
        let clean_snippet = snippet.trim().to_string();
        if uri.is_empty() || clean_snippet.is_empty() {
            return;
        }

        let hit = SessionSearchHit {
            uri,
            memory_id,
            snippet: clean_snippet,
            updated_at: updated_at.unwrap_or_else(utc_iso_now),
            priority,
            source,
        };

        let mut hits = self.hits.write().await;
        let queue = hits.entry(sid.clone()).or_insert_with(VecDeque::new);
        queue.push_back(hit);
        if queue.len() > 200 {
            queue.pop_front();
        }

        let mut last_seen = self.session_last_seen.write().await;
        let mut seq = self.touch_sequence.lock().unwrap();
        *seq += 1;
        last_seen.insert(sid, (Utc::now().timestamp() as f64, *seq));
    }

    pub async fn search(&self, session_id: Option<String>, query: &str, limit: usize) -> Vec<HashMap<String, serde_json::Value>> {
        let sid = session_id.unwrap_or_else(|| "default".to_string());
        let hits = self.hits.read().await;
        let Some(queue) = hits.get(&sid) else {
            return vec![];
        };

        let terms: Vec<&str> = query.to_lowercase().split_whitespace().collect();
        let mut by_uri = HashMap::new();

        for item in queue {
            let text = item.snippet.to_lowercase();
            let hits_count = terms.iter().filter(|&&t| text.contains(t)).count();
            if hits_count == 0 {
                continue;
            }

            let score = hits_count as f64 / terms.len() as f64;
            let candidate = serde_json::json!({
                "uri": item.uri,
                "memory_id": item.memory_id,
                "snippet": &item.snippet[..item.snippet.len().min(300)],
                "priority": item.priority,
                "score": score,
                "updated_at": item.updated_at,
                "source": item.source,
            });

            by_uri.insert(item.uri.clone(), candidate);
        }

        let mut ranked: Vec<_> = by_uri.into_values().collect();
        ranked.sort_by(|a, b| {
            let a_score = a["score"].as_f64().unwrap_or(0.0);
            let b_score = b["score"].as_f64().unwrap_or(0.0);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });

        ranked.into_iter().take(limit).collect()
    }
}

// ==============================
// 2. 会话刷新追踪器
// ==============================
pub struct SessionFlushTracker {
    events: Arc<RwLock<HashMap<String, VecDeque<String>>>>,
    session_last_seen: Arc<RwLock<HashMap<String, (f64, u64)>>>,
    touch_sequence: Arc<Mutex<u64>>,
}

impl SessionFlushTracker {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
            session_last_seen: Arc::new(RwLock::new(HashMap::new())),
            touch_sequence: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn record_event(&self, session_id: Option<String>, message: String) {
        let text = message.trim().to_string();
        if text.is_empty() {
            return;
        }
        let sid = session_id.unwrap_or_else(|| "default".to_string());

        let mut events = self.events.write().await;
        let queue = events.entry(sid.clone()).or_insert_with(VecDeque::new);
        queue.push_back(text[..text.len().min(400)].to_string());
        if queue.len() > 80 {
            queue.pop_front();
        }

        let mut last_seen = self.session_last_seen.write().await;
        let mut seq = self.touch_sequence.lock().unwrap();
        *seq += 1;
        last_seen.insert(sid, (Utc::now().timestamp() as f64, *seq));
    }

    pub async fn should_flush(&self, session_id: Option<String>) -> bool {
        let sid = session_id.unwrap_or_else(|| "default".to_string());
        let events = self.events.read().await;
        let Some(queue) = events.get(&sid) else {
            return false;
        };
        queue.len() >= 6 && queue.iter().map(|s| s.len()).sum::<usize>() >= 6000
    }

    pub async fn build_summary(&self, session_id: Option<String>, limit: usize) -> String {
        let sid = session_id.unwrap_or_else(|| "default".to_string());
        let events = self.events.read().await;
        let Some(queue) = events.get(&sid) else {
            return "".to_string();
        };

        let tail: Vec<_> = queue.iter().rev().take(limit).rev().collect();
        let lines: Vec<_> = tail.iter().map(|line| format!("- {}", line)).collect();
        format!("Session compaction notes:\n{}", lines.join("\n"))
    }
}

// ==============================
// 3. 守卫决策追踪器
// ==============================
pub struct GuardDecisionTracker {
    events: Arc<RwLock<VecDeque<GuardDecisionEvent>>>,
}

impl GuardDecisionTracker {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(VecDeque::with_capacity(300))),
        }
    }

    pub async fn record_event(
        &self,
        operation: String,
        action: String,
        method: String,
        reason: String,
        target_id: Option<i64>,
        blocked: bool,
        degraded: bool,
        degrade_reasons: Vec<String>,
    ) {
        let event = GuardDecisionEvent {
            timestamp: utc_iso_now(),
            operation: operation.trim().to_string(),
            action: action.trim().to_uppercase(),
            method: method.trim().to_lowercase(),
            reason: reason.trim().to_string(),
            target_id,
            blocked,
            degraded,
            degrade_reasons,
        };

        let mut events = self.events.write().await;
        events.push_back(event);
        if events.len() > 300 {
            events.pop_front();
        }
    }

    pub async fn summary(&self) -> serde_json::Value {
        let events = self.events.read().await;
        if events.is_empty() {
            return serde_json::json!({
                "window_size": 300,
                "total_events": 0,
                "blocked_events": 0,
                "degraded_events": 0,
                "action_breakdown": {},
                "method_breakdown": {},
                "operation_breakdown": {},
                "top_reasons": [],
                "last_event_at": None,
            });
        }

        let mut action_counter = HashMap::new();
        let mut method_counter = HashMap::new();
        let mut operation_counter = HashMap::new();
        let mut reason_counter = HashMap::new();
        let mut blocked_events = 0;
        let mut degraded_events = 0;

        for event in events.iter() {
            *action_counter.entry(event.action.clone()).or_insert(0) += 1;
            *method_counter.entry(event.method.clone()).or_insert(0) += 1;
            *operation_counter.entry(event.operation.clone()).or_insert(0) += 1;
            if !event.reason.is_empty() {
                *reason_counter.entry(event.reason.clone()).or_insert(0) += 1;
            }
            if event.blocked {
                blocked_events += 1;
            }
            if event.degraded {
                degraded_events += 1;
            }
        }

        let mut top_reasons: Vec<_> = reason_counter.into_iter().collect();
        top_reasons.sort_by(|a, b| b.1.cmp(&a.1));
        let top_reasons: Vec<_> = top_reasons
            .into_iter()
            .take(5)
            .map(|(reason, count)| serde_json::json!({"reason": reason, "count": count}))
            .collect();

        serde_json::json!({
            "window_size": 300,
            "total_events": events.len(),
            "blocked_events": blocked_events,
            "degraded_events": degraded_events,
            "action_breakdown": action_counter,
            "method_breakdown": method_counter,
            "operation_breakdown": operation_counter,
            "top_reasons": top_reasons,
            "last_event_at": events.back().unwrap().timestamp,
        })
    }
}

// ==============================
// 4. 清理确认协调器
// ==============================
#[derive(Debug, Clone)]
pub struct CleanupReviewRecord {
    pub review_id: String,
    pub token: String,
    pub confirmation_phrase: String,
    pub action: String,
    pub reviewer: String,
    pub selections: Vec<serde_json::Value>,
    pub created_at: f64,
    pub expires_at: f64,
}

pub struct CleanupReviewCoordinator {
    records: Arc<RwLock<HashMap<String, CleanupReviewRecord>>>,
}

impl CleanupReviewCoordinator {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_review(
        &self,
        action: String,
        selections: Vec<serde_json::Value>,
        reviewer: Option<String>,
        ttl_seconds: Option<i64>,
    ) -> serde_json::Value {
        let normalized_action = action.trim().to_lowercase();
        let normalized_reviewer = reviewer.unwrap_or_else(|| "human".to_string());
        let ttl = ttl_seconds.unwrap_or(900).max(60) as f64;
        let now_ts = Utc::now().timestamp() as f64;
        let expires_at = now_ts + ttl;
        let review_id = format!("cleanup-{}", Uuid::new_v4().to_string().replace("-", "")[..10].to_string());
        let token = Uuid::new_v4().to_string().replace("-", "");
        let confirmation_phrase = format!("CONFIRM {} {}", normalized_action.to_uppercase(), selections.len());

        let record = CleanupReviewRecord {
            review_id: review_id.clone(),
            token: token.clone(),
            confirmation_phrase: confirmation_phrase.clone(),
            action: normalized_action,
            reviewer: normalized_reviewer,
            selections,
            created_at: now_ts,
            expires_at,
        };

        let mut records = self.records.write().await;
        records.insert(review_id.clone(), record);

        serde_json::json!({
            "review_id": review_id,
            "token": token,
            "confirmation_phrase": confirmation_phrase,
            "action": normalized_action,
            "reviewer": normalized_reviewer,
            "expires_at": DateTime::from_timestamp(expires_at as i64, 0).unwrap().to_rfc3339(),
        })
    }

    pub async fn consume_review(
        &self,
        review_id: String,
        token: String,
        confirmation_phrase: String,
    ) -> serde_json::Value {
        let review_id_value = review_id.trim();
        let token_value = token.trim();
        let phrase_value = confirmation_phrase.trim();

        if review_id_value.is_empty() {
            return serde_json::json!({"ok": false, "error": "review_id is required"});
        }
        if token_value.is_empty() {
            return serde_json::json!({"ok": false, "error": "token is required"});
        }
        if phrase_value.is_empty() {
            return serde_json::json!({"ok": false, "error": "confirmation_phrase is required"});
        }

        let mut records = self.records.write().await;
        let Some(record) = records.remove(review_id_value) else {
            return serde_json::json!({"ok": false, "error": "review_not_found_or_expired"});
        };

        if record.token != token_value {
            return serde_json::json!({"ok": false, "error": "invalid_review_token"});
        }
        if record.confirmation_phrase != phrase_value {
            return serde_json::json!({"ok": false, "error": "confirmation_phrase_mismatch"});
        }

        serde_json::json!({
            "ok": true,
            "review": {
                "review_id": record.review_id,
                "action": record.action,
                "reviewer": record.reviewer,
                "selections": record.selections,
                "created_at": DateTime::from_timestamp(record.created_at as i64, 0).unwrap().to_rfc3339(),
                "expires_at": DateTime::from_timestamp(record.expires_at as i64, 0).unwrap().to_rfc3339(),
            },
        })
    }
}

// ==============================
// 5. 主运行时状态（整合所有模块）
// ==============================
pub struct RuntimeState {
    pub session_cache: SessionSearchCache,
    pub flush_tracker: SessionFlushTracker,
    pub guard_tracker: GuardDecisionTracker,
    pub cleanup_reviews: CleanupReviewCoordinator,
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            session_cache: SessionSearchCache::new(),
            flush_tracker: SessionFlushTracker::new(),
            guard_tracker: GuardDecisionTracker::new(),
            cleanup_reviews: CleanupReviewCoordinator::new(),
        }
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}