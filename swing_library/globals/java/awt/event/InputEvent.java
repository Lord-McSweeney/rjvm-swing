package java.awt.event;

import java.awt.Component;

public abstract class InputEvent extends ComponentEvent {
    InputEvent(Component source, int id, long when, int modifiers) {
        super(source, id);
    }
}
