package java.awt.event;

import java.awt.Component;

public class KeyEvent extends InputEvent {
    private int keyCode;

    public KeyEvent(Component source, int id, long when, int modifiers, int keyCode, char keyChar, int keyLocation) {
        super(source, id, when, modifiers);
        this.keyCode = keyCode;
    }

    public KeyEvent(Component source, int id, long when, int modifiers, int keyCode, char keyChar) {
        this(source, id, when, modifiers, keyCode, keyChar, 0);
    }

    public KeyEvent(Component source, int id, long when, int modifiers, int keyCode) {
        this(source, id, when, modifiers, keyCode, (char) 0);
    }

    public int getKeyCode() {
        return this.keyCode;
    }
}
