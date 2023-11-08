package sh.cocaine.sexo;

public class BotUser {

    String hostmask;
    byte level;

    boolean autoOp;
    boolean autoVoice;

    public BotUser(String hostmask, byte level, boolean autoOp, boolean autoVoice) {
        this.hostmask = hostmask;
        this.level = level;
        this.autoOp = autoOp;
        this.autoVoice = autoVoice;
    }
}