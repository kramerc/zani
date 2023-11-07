package sh.cocaine.sexo;

public class BotUser {

    String hostmask;
    byte level;

    boolean autoOp;

    public BotUser(String hostmask, byte level, boolean autoOp) {
        this.hostmask = hostmask;
        this.level = level;
        this.autoOp = autoOp;
    }
}