package java.awt.event;

import java.awt.Window;

public class WindowEvent extends ComponentEvent {
    public WindowEvent(Window source, int id) {
        super(source, id);
    }
}
