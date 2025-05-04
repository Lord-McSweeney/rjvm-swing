package java.awt.event;

import java.awt.Component;

public class KeyEvent extends InputEvent {
    public KeyEvent(Component source, int id, long when, int modifiers, int keyCode, char keyChar, int keyLocation) {
        super(source, id, when, modifiers);
    }

    public KeyEvent(Component source, int id, long when, int modifiers, int keyCode, char keyChar) {
        this(source, id, when, modifiers, keyCode, keyChar, 0);
    }
}
