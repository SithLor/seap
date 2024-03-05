package exs.shared;
public class StringBuilder {
    private char[] chars;
    private int size;

    public StringBuilder() {
        chars = new char[16];
        size = 0;
    }

    public StringBuilder append(String str) {
        if (str == null) {
            str = "null";
        }
        int len = str.length();
        ensureCapacity(size + len);
        str.getChars(0, len, chars, size);
        size += len;
        return this;
    }

    public String toString() {
        return new String(chars, 0, size);
    }

    private void ensureCapacity(int minimumCapacity) {
        if (minimumCapacity - chars.length > 0) {
            expandCapacity(minimumCapacity);
        }
    }

    private void expandCapacity(int minimumCapacity) {
        int newCapacity = chars.length * 2 + 2;
        if (newCapacity - minimumCapacity < 0) {
            newCapacity = minimumCapacity;
        }
        char[] copy = new char[newCapacity];
        System.arraycopy(chars, 0, copy, 0, size);
        chars = copy;
    }
}