package java.awt.event;

import java.awt.Component;

public class MouseEvent extends InputEvent {
    public MouseEvent(Component source, int id, long when, int modifiers, int x, int y, int xAbs, int yAbs, int clickCount, boolean popupTrigger, int button) {
        super(source, id, when, modifiers);
    }
}
