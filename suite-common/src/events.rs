// events.rs — Generic observer/broadcaster pattern.
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Pattern: LibreOffice svl/ SfxBroadcaster + SfxListener.
// Provides typed event broadcasting for undo state changes,
// selection changes, and other app-level notifications.

use std::cell::RefCell;
use std::rc::Rc;

/// A listener that receives hints of type H.
pub trait Listener<H> {
    fn on_event(&self, hint: &H);
}

/// Generic broadcaster: maintains a list of listeners and notifies them.
pub struct Broadcaster<H> {
    listeners: RefCell<Vec<Rc<dyn Listener<H>>>>,
}

impl<H> Broadcaster<H> {
    pub fn new() -> Self {
        Broadcaster { listeners: RefCell::new(Vec::new()) }
    }

    pub fn listen(&self, listener: Rc<dyn Listener<H>>) {
        self.listeners.borrow_mut().push(listener);
    }

    pub fn broadcast(&self, hint: H) {
        for listener in self.listeners.borrow().iter() {
            listener.on_event(&hint);
        }
    }
}

impl<H> Default for Broadcaster<H> {
    fn default() -> Self { Self::new() }
}

// ── Predefined hint types for suite-level events ──────────────────────

#[derive(Clone, Debug)]
pub enum Hint {
    /// Undo/redo stack state changed.
    UndoStateChanged { can_undo: bool, can_redo: bool },
    /// Active slide changed.
    SlideSwitched { from: usize, to: usize },
    /// Cell selection moved.
    CellSelected { sheet: usize, row: usize, col: usize },
    /// Document dirty flag toggled.
    DocumentModified { dirty: bool },
}
