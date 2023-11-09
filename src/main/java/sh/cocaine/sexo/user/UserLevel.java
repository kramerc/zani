package sh.cocaine.sexo.user;

public enum UserLevel {
    USER(0),
    OP(1),
    ADMIN(3);

    private final byte level;

    UserLevel(int level) {
        this.level = (byte) level;
    }

    public byte getLevel() {
        return level;
    }
}
