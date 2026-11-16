//! Editor actions and key bindings (behavior-spec §4.2).

use gpui::*;

actions!(
    editor,
    [
        MoveLeft,
        MoveRight,
        MoveUp,
        MoveDown,
        SelectLeft,
        SelectRight,
        SelectUp,
        SelectDown,
        WordLeft,
        WordRight,
        SelectWordLeft,
        SelectWordRight,
        LineStart,
        LineEnd,
        SelectLineStart,
        SelectLineEnd,
        DocStart,
        DocEnd,
        SelectToDocStart,
        SelectToDocEnd,
        Backspace,
        Delete,
        DeleteWordBack,
        DeleteWordForward,
        DeleteToLineStart,
        Enter,
        SoftBreak,
        ToggleTodo,
        Tab,
        ShiftTab,
        Escape,
        SelectAll,
        Copy,
        CopyMarkdown,
        Cut,
        Paste,
        PastePlain,
        Undo,
        Redo,
        Save,
        Bold,
        Italic,
        Underline,
        Strike,
        InlineCode,
        Link,
        ApplyLastColor,
        InsertReminder,
        TurnText,
        TurnH1,
        TurnH2,
        TurnH3,
        TurnH4,
        TurnH5,
        TurnH6,
        TurnH7,
        TurnH8,
        TurnH9,
        TurnOrdered,
        TurnBullet,
        TurnTodo,
        TurnCode,
        TurnQuote,
        InsertDivider,
        InsertTable,
        AlignLeft,
        AlignCenter,
        AlignRight,
        MoveBlockUp,
        MoveBlockDown,
        // Vim: enter Insert mode from Normal (behave like nvim's i/a/I/A/o/O).
        InsertHere,
        InsertAfter,
        InsertLineStart,
        InsertLineEnd,
        OpenBelow,
        OpenAbove,
        // Vim Normal operators (motions reuse the Move*/Word*/Line*/Doc* actions).
        VimDeleteChar,   // x
        VimOpDelete,     // d (then a motion, or d again for dd)
        VimOpChange,     // c
        VimOpYank,       // y
        VimVisual,       // v (toggle charwise Visual)
        VimVisualLine,   // V (toggle line-wise Visual)
        VimVisualBlock,  // ctrl-v (toggle block-wise Visual)
        VimVisualDelete, // d / x in Visual
        VimVisualYank,   // y in Visual
        VimVisualBlockDelete, // d / x in Visual Block
        VimVisualBlockYank,   // y in Visual Block
        VimBlockDown,         // j in Visual Block (column-preserving)
        VimBlockUp,           // k in Visual Block (column-preserving)
        VimReplace,           // r (replace the char under the caret)
        VimCommand,           // : (ex command line)
        // Vim text objects: `i`/`a` after an operator wait for the object key.
        VimTextObjInner,      // i (after d/c/y)
        VimTextObjAround,     // a (after d/c/y)
        VimObjWord,           // w
        VimObjPara,           // p
        VimObjQuoteD,         // "
        VimObjQuoteS,         // '
        VimObjQuoteBt,        // `
        VimObjParen,          // ( ) b
        VimObjBrace,          // { } B
        VimObjBracket,        // [ ]
        VimObjAngle,          // < >
        // Vim search, over the existing find engine (editor/find.rs).
        VimSearchOpen,        // / ?
        VimSearchNext,        // n
        VimSearchPrev,        // shift-n
        VimSearchWordFwd,     // * (and g *)
        VimSearchWordBack,    // # (and g #)
    ]
);

pub fn bind_keys(cx: &mut App) {
    // Moving, selecting and copying also work in 阅读模式 ("EditorRO"),
    // which binds nothing that edits.
    for c in [Some("Editor"), Some("EditorRO")] {
        cx.bind_keys([
            KeyBinding::new("left", MoveLeft, c),
            KeyBinding::new("right", MoveRight, c),
            KeyBinding::new("up", MoveUp, c),
            KeyBinding::new("down", MoveDown, c),
            KeyBinding::new("ctrl-b", MoveLeft, c),
            KeyBinding::new("ctrl-f", MoveRight, c),
            KeyBinding::new("ctrl-p", MoveUp, c),
            KeyBinding::new("ctrl-n", MoveDown, c),
            KeyBinding::new("shift-left", SelectLeft, c),
            KeyBinding::new("shift-right", SelectRight, c),
            KeyBinding::new("shift-up", SelectUp, c),
            KeyBinding::new("shift-down", SelectDown, c),
            KeyBinding::new("alt-left", WordLeft, c),
            KeyBinding::new("alt-right", WordRight, c),
            KeyBinding::new("alt-shift-left", SelectWordLeft, c),
            KeyBinding::new("alt-shift-right", SelectWordRight, c),
            KeyBinding::new("cmd-left", LineStart, c),
            KeyBinding::new("cmd-right", LineEnd, c),
            KeyBinding::new("home", LineStart, c),
            KeyBinding::new("end", LineEnd, c),
            KeyBinding::new("ctrl-a", LineStart, c),
            KeyBinding::new("ctrl-e", LineEnd, c),
            KeyBinding::new("cmd-shift-left", SelectLineStart, c),
            KeyBinding::new("cmd-shift-right", SelectLineEnd, c),
            KeyBinding::new("shift-home", SelectLineStart, c),
            KeyBinding::new("shift-end", SelectLineEnd, c),
            KeyBinding::new("cmd-up", DocStart, c),
            KeyBinding::new("cmd-down", DocEnd, c),
            KeyBinding::new("cmd-shift-up", SelectToDocStart, c),
            KeyBinding::new("cmd-shift-down", SelectToDocEnd, c),
            KeyBinding::new("escape", Escape, c),
            KeyBinding::new("cmd-a", SelectAll, c),
            KeyBinding::new("cmd-c", Copy, c),
            KeyBinding::new("cmd-s", Save, c),
        ]);
    }
    let c = Some("Editor");
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, c),
        KeyBinding::new("shift-backspace", Backspace, c),
        KeyBinding::new("ctrl-backspace", Backspace, c),
        KeyBinding::new("ctrl-h", Backspace, c),
        KeyBinding::new("delete", Delete, c),
        KeyBinding::new("ctrl-d", Delete, c),
        KeyBinding::new("alt-backspace", DeleteWordBack, c),
        KeyBinding::new("alt-delete", DeleteWordForward, c),
        KeyBinding::new("cmd-backspace", DeleteToLineStart, c),
        KeyBinding::new("enter", Enter, c),
        KeyBinding::new("shift-enter", SoftBreak, c),
        KeyBinding::new("cmd-enter", ToggleTodo, c),
        KeyBinding::new("tab", Tab, c),
        KeyBinding::new("shift-tab", ShiftTab, c),
        KeyBinding::new("cmd-x", Cut, c),
        KeyBinding::new("cmd-v", Paste, c),
        KeyBinding::new("cmd-shift-v", PastePlain, c),
        KeyBinding::new("cmd-shift-c", CopyMarkdown, c),
        KeyBinding::new("cmd-z", Undo, c),
        KeyBinding::new("cmd-shift-z", Redo, c),
        KeyBinding::new("cmd-y", Redo, c),
        KeyBinding::new("cmd-b", Bold, c),
        KeyBinding::new("cmd-i", Italic, c),
        KeyBinding::new("cmd-u", Underline, c),
        KeyBinding::new("cmd-shift-x", Strike, c),
        KeyBinding::new("cmd-ctrl-c", InlineCode, c),
        KeyBinding::new("cmd-k", Link, c),
        KeyBinding::new("cmd-alt-h", ApplyLastColor, c),
        KeyBinding::new("cmd-alt-w", InsertReminder, c),
        KeyBinding::new("cmd-alt-0", TurnText, c),
        KeyBinding::new("cmd-alt-1", TurnH1, c),
        KeyBinding::new("cmd-alt-2", TurnH2, c),
        KeyBinding::new("cmd-alt-3", TurnH3, c),
        KeyBinding::new("cmd-alt-4", TurnH4, c),
        KeyBinding::new("cmd-alt-5", TurnH5, c),
        KeyBinding::new("cmd-alt-6", TurnH6, c),
        KeyBinding::new("cmd-alt-7", TurnH7, c),
        KeyBinding::new("cmd-alt-8", TurnH8, c),
        KeyBinding::new("cmd-alt-9", TurnH9, c),
        KeyBinding::new("cmd-shift-7", TurnOrdered, c),
        KeyBinding::new("cmd-&", TurnOrdered, c),
        KeyBinding::new("cmd-shift-8", TurnBullet, c),
        KeyBinding::new("cmd-*", TurnBullet, c),
        KeyBinding::new("cmd-alt-t", TurnTodo, c),
        KeyBinding::new("cmd-alt-c", TurnCode, c),
        KeyBinding::new("cmd-shift-.", TurnQuote, c),
        KeyBinding::new("cmd->", TurnQuote, c),
        KeyBinding::new("cmd-alt-s", InsertDivider, c),
        KeyBinding::new("cmd-alt-a", InsertTable, c),
        KeyBinding::new("cmd-shift-l", AlignLeft, c),
        KeyBinding::new("cmd-shift-e", AlignCenter, c),
        KeyBinding::new("cmd-shift-r", AlignRight, c),
        KeyBinding::new("ctrl-shift-up", MoveBlockUp, c),
        KeyBinding::new("ctrl-shift-down", MoveBlockDown, c),
    ]);
    // PC clipboard keys (Ctrl+Insert after `shortcuts::pc_binding`).
    #[cfg(not(target_os = "macos"))]
    cx.bind_keys([
        KeyBinding::new("shift-delete", Cut, c),
        KeyBinding::new("cmd-insert", Copy, c),
        KeyBinding::new("shift-insert", Paste, c),
    ]);

    // Vim Normal mode: letters are commands. These bindings only match when the
    // editor's key context carries `vim = normal` (set in render.rs when the
    // Vim-mode setting is on), so default editing is untouched when it is off.
    let vn = Some("Editor && vim == normal");
    cx.bind_keys([
        // Enter Insert.
        KeyBinding::new("i", InsertHere, vn),
        KeyBinding::new("a", InsertAfter, vn),
        KeyBinding::new("shift-i", InsertLineStart, vn),
        KeyBinding::new("shift-a", InsertLineEnd, vn),
        KeyBinding::new("o", OpenBelow, vn),
        KeyBinding::new("shift-o", OpenAbove, vn),
        // Motions (reuse the existing caret actions).
        KeyBinding::new("h", MoveLeft, vn),
        KeyBinding::new("l", MoveRight, vn),
        KeyBinding::new("j", MoveDown, vn),
        KeyBinding::new("k", MoveUp, vn),
        KeyBinding::new("w", WordRight, vn),
        KeyBinding::new("b", WordLeft, vn),
        KeyBinding::new("e", WordRight, vn),
        KeyBinding::new("0", LineStart, vn),
        KeyBinding::new("$", LineEnd, vn),
        KeyBinding::new("g g", DocStart, vn),
        KeyBinding::new("shift-g", DocEnd, vn),
        // Search.
        KeyBinding::new("/", VimSearchOpen, vn),
        KeyBinding::new("?", VimSearchOpen, vn),
        KeyBinding::new("n", VimSearchNext, vn),
        KeyBinding::new("shift-n", VimSearchPrev, vn),
        KeyBinding::new("*", VimSearchWordFwd, vn),
        KeyBinding::new("#", VimSearchWordBack, vn),
        KeyBinding::new("g *", VimSearchWordFwd, vn),
        KeyBinding::new("g #", VimSearchWordBack, vn),
        // Operators (then a motion; pressed twice = line-wise dd/cc/yy) and mode.
        KeyBinding::new("x", VimDeleteChar, vn),
        KeyBinding::new("d", VimOpDelete, vn),
        KeyBinding::new("c", VimOpChange, vn),
        KeyBinding::new("y", VimOpYank, vn),
        KeyBinding::new("p", Paste, vn),
        KeyBinding::new("u", Undo, vn),
        KeyBinding::new("v", VimVisual, vn),
        KeyBinding::new("shift-v", VimVisualLine, vn),
        KeyBinding::new("r", VimReplace, vn),
        KeyBinding::new(":", VimCommand, vn),
        // ctrl-v (cmd-v, PC-mapped to ctrl-v) enters Visual Block; in Insert or
        // with Vim off it falls through to the Editor binding and pastes.
        KeyBinding::new("cmd-v", VimVisualBlock, vn),
    ]);

    // Vim text objects: `i`/`a` only mean "inner"/"around" while an operator
    // is pending (render.rs adds `pending` to the context then); these are
    // registered after `vn` so they win the tie (same depth, later wins) and
    // override `i`→InsertHere / `a`→InsertAfter / `w`→WordRight / `p`→Paste /
    // `b`→WordLeft for that one keystroke. Off the pending path, `vn`'s own
    // bindings still win (the extra `&&` clause fails to match).
    let vn_pending = Some("Editor && vim == normal && pending");
    cx.bind_keys([
        KeyBinding::new("i", VimTextObjInner, vn_pending),
        KeyBinding::new("a", VimTextObjAround, vn_pending),
    ]);
    let vn_textobj = Some("Editor && vim == normal && pending && textobj");
    cx.bind_keys([
        KeyBinding::new("w", VimObjWord, vn_textobj),
        KeyBinding::new("p", VimObjPara, vn_textobj),
        KeyBinding::new("\"", VimObjQuoteD, vn_textobj),
        KeyBinding::new("'", VimObjQuoteS, vn_textobj),
        KeyBinding::new("`", VimObjQuoteBt, vn_textobj),
        KeyBinding::new("(", VimObjParen, vn_textobj),
        KeyBinding::new(")", VimObjParen, vn_textobj),
        KeyBinding::new("b", VimObjParen, vn_textobj),
        KeyBinding::new("{", VimObjBrace, vn_textobj),
        KeyBinding::new("}", VimObjBrace, vn_textobj),
        KeyBinding::new("shift-b", VimObjBrace, vn_textobj),
        KeyBinding::new("[", VimObjBracket, vn_textobj),
        KeyBinding::new("]", VimObjBracket, vn_textobj),
        KeyBinding::new("<", VimObjAngle, vn_textobj),
        KeyBinding::new(">", VimObjAngle, vn_textobj),
    ]);

    // Vim Visual mode: motions extend the selection; d/y act on it then exit.
    let vv = Some("Editor && vim == visual");
    cx.bind_keys([
        KeyBinding::new("h", SelectLeft, vv),
        KeyBinding::new("l", SelectRight, vv),
        KeyBinding::new("j", SelectDown, vv),
        KeyBinding::new("k", SelectUp, vv),
        KeyBinding::new("w", SelectWordRight, vv),
        KeyBinding::new("b", SelectWordLeft, vv),
        KeyBinding::new("e", SelectWordRight, vv),
        KeyBinding::new("0", SelectLineStart, vv),
        KeyBinding::new("$", SelectLineEnd, vv),
        KeyBinding::new("g g", SelectToDocStart, vv),
        KeyBinding::new("shift-g", SelectToDocEnd, vv),
        KeyBinding::new("d", VimVisualDelete, vv),
        KeyBinding::new("x", VimVisualDelete, vv),
        KeyBinding::new("y", VimVisualYank, vv),
        KeyBinding::new("v", VimVisual, vv),
        KeyBinding::new("shift-v", VimVisualLine, vv),
        KeyBinding::new("cmd-v", VimVisualBlock, vv),
    ]);

    // Vim Visual Block: motions grow a columnar selection; d/y act on the
    // rectangle. The paste shortcut (ctrl-v) toggles the mode back off.
    let vb = Some("Editor && vim == visualblock");
    cx.bind_keys([
        KeyBinding::new("h", SelectLeft, vb),
        KeyBinding::new("l", SelectRight, vb),
        // Column-preserving vertical motion (not SelectDown/Up, which jump to
        // the line edge and then into whole-block selection).
        KeyBinding::new("j", VimBlockDown, vb),
        KeyBinding::new("k", VimBlockUp, vb),
        KeyBinding::new("w", SelectWordRight, vb),
        KeyBinding::new("b", SelectWordLeft, vb),
        KeyBinding::new("0", SelectLineStart, vb),
        KeyBinding::new("$", SelectLineEnd, vb),
        KeyBinding::new("d", VimVisualBlockDelete, vb),
        KeyBinding::new("x", VimVisualBlockDelete, vb),
        KeyBinding::new("y", VimVisualBlockYank, vb),
        KeyBinding::new("v", VimVisual, vb),
        KeyBinding::new("shift-v", VimVisualLine, vb),
        KeyBinding::new("cmd-v", VimVisualBlock, vb),
    ]);
}
