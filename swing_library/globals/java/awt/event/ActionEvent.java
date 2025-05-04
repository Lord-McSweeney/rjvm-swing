package java.awt.event;

import java.awt.AWTEvent;

public class ActionEvent extends AWTEvent {
    public ActionEvent(Object source, int id, String command) {
        super(source, id);
    }
}
