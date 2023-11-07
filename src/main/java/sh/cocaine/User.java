package sh.cocaine;

public class User {

    String hostmask;
    byte level;

    boolean autoOp;

    public User(String hostmask, byte level, boolean autoOp) {
        this.hostmask = hostmask;
        this.level = level;
        this.autoOp = autoOp;
    }
}