// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
Structs and helper methods for messaging
*/
use chrono::prelude::*;
use queues::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    INFO,
    WARNING,
    ERROR,
}

/// Implements fmt::Debug for MessageType enum
///
/// # Arguments
///
/// * `f` - A fmt::Formatter
///
///
/// # Returns
///
/// * `fmt::Result` - Result of this method
///
///
#[cfg(not(tarpaulin_include))]
impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// This is a struct for our baselines, so callers know the antenna ordering
#[allow(non_camel_case_types)]
pub struct mwalibMessageQueue {
    messages: Queue<mwalibMessage>,
}

impl mwalibMessageQueue {
    /// Creates a new, populated mwalibMessageQueue struct for storing all the messages
    ///            
    /// # Returns
    ///
    /// * A populated mwalibMessageQueue struct
    ///    
    pub fn new() -> Self {
        Self {
            messages: Queue::new(),
        }
    }

    /// Gets the next message or None if queue is empty
    ///            
    /// # Returns
    ///
    /// * Returns mwalibMessage or None
    ///    
    pub fn get_next_message(&mut self) -> Option<mwalibMessage> {
        match self.messages.size() {
            x if x > 0 => Some(self.messages.remove().unwrap()),
            _ => None,
        }
    }

    /// Gets count of messages on the queue
    ///            
    /// # Returns
    ///
    /// * Returns count of messages
    ///    
    pub fn size(&self) -> usize {
        self.messages.size()
    }

    /// Creates a new INFO message and adds it to the queue
    ///
    /// # Arguments
    ///    
    /// * `message_text` - The text of the message    
    ///
    ///
    /// # Returns
    ///
    /// Nothing
    ///    
    pub fn info_message(&mut self, message_text: &str) {
        self.messages
            .add(mwalibMessage {
                timestamp: Local::now(),
                message_type: MessageType::INFO,
                message_text: message_text.to_string(),
            })
            .expect("Error adding info message to queue");
    }

    /// Creates a new WARNING message and adds it to the queue
    ///
    /// # Arguments
    ///    
    /// * `message_text` - The text of the message    
    ///
    ///
    /// # Returns
    ///
    /// Nothing
    ///    
    pub fn warning_message(&mut self, message_text: &str) {
        self.messages
            .add(mwalibMessage {
                timestamp: Local::now(),
                message_type: MessageType::WARNING,
                message_text: message_text.to_string(),
            })
            .expect("Error adding warning message to queue");
    }

    /// Creates a new ERROR message and adds it to the queue
    ///
    /// # Arguments
    ///    
    /// * `message_text` - The text of the message    
    ///
    ///
    /// # Returns
    ///
    /// Nothing
    ///    
    pub fn error_message(&mut self, message_text: &str) {
        self.messages
            .add(mwalibMessage {
                timestamp: Local::now(),
                message_type: MessageType::ERROR,
                message_text: message_text.to_string(),
            })
            .expect("Error adding error message to queue");
    }
}

/// Implements fmt::Debug for mwalibMessageQueue
///
/// # Arguments
///
/// * `f` - A fmt::Formatter
///
///
/// # Returns
///
/// * `fmt::Result` - Result of this method
///
///
#[cfg(not(tarpaulin_include))]
impl fmt::Debug for mwalibMessageQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} messages", self.messages.size())
    }
}

/// This is a struct for a single message
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct mwalibMessage {
    /// The date/time of the message
    pub timestamp: DateTime<Local>,
    /// The type of message
    pub message_type: MessageType,
    /// The text of the message
    pub message_text: String,
}

/// Implements fmt::Debug for mwalibMessage struct
///
/// # Arguments
///
/// * `f` - A fmt::Formatter
///
///
/// # Returns
///
/// * `fmt::Result` - Result of this method
///
///
#[cfg(not(tarpaulin_include))]
impl fmt::Debug for mwalibMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.timestamp,
            self.message_type.to_string(),
            self.message_text,
        )
    }
}
