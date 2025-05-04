package java.awt;

import java.util.EventObject;

public abstract class AWTEvent extends EventObject {
    public AWTEvent(Object source, int id) {
        super(source);
    }
}
