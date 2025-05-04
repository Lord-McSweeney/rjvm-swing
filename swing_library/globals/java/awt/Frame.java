package java.awt;

public class Frame extends Window {
    public Frame(String name) {
        super(null);

        Frame.initName(name);
    }

    private static native void initName(String name);
}
