package java.awt;

public abstract class Graphics2D extends Graphics {
    public abstract void rotate(double theta);
}

class CRC2DGraphics extends Graphics2D {
    private Font font;

    CRC2DGraphics() {
        super();
    }

    public native void drawLine(int x1, int y1, int x2, int y2);

    public native void drawString(String str, int x, int y);

    public native void fillRect(int x, int y, int width, int height);

    public Font getFont() {
        return this.font;
    }

    public FontMetrics getFontMetrics(Font font) {
        // TODO implement
        return new FontMetrics(null);
    }

    public native void setColor(Color color);

    public void setFont(Font font) {
        this.font = font;
        this.internalSetFont(font.name, font.size, font.style);
    }

    private native void internalSetFont(String name, int size, int style);

    public native void translate(int x, int y);

    public native void rotate(double theta);
}
