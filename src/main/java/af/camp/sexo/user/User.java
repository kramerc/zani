package af.camp.sexo.user;

import af.camp.sexo.db.DatabaseEngine;

import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.util.logging.Level;
import java.util.logging.Logger;

public class User {

    private static final Logger logger = Logger.getLogger(User.class.getName());

    private Integer id;

    private String hostmask;

    private byte level;

    private boolean autoOp;

    private boolean autoVoice;

    public User(int id, String hostmask, byte level, boolean autoOp, boolean autoVoice) {
        this.id = id;
        this.hostmask = hostmask;
        this.level = level;
        this.autoOp = autoOp;
        this.autoVoice = autoVoice;
    }

    public User(String hostmask, byte level, boolean autoOp, boolean autoVoice) {
        this.hostmask = hostmask;
        this.level = level;
        this.autoOp = autoOp;
        this.autoVoice = autoVoice;
    }

    public static User findByHostmask(String hostmask) {
        // Query the database for the hostmask
        try {
            PreparedStatement stmt = DatabaseEngine.getInstance().getConnection().prepareStatement("SELECT * FROM users WHERE hostmask = ? LIMIT 1");
            stmt.setString(1, hostmask);

            ResultSet rs = stmt.executeQuery();
            User user = null;

            if (rs.next()) {
                user = new User(
                        rs.getInt("id"),
                        rs.getString("hostmask"),
                        rs.getByte("level"),
                        rs.getBoolean("autoOp"),
                        rs.getBoolean("autoVoice")
                );
            }

            rs.close();
            stmt.close();

            return user;
        } catch (SQLException e) {
            logger.log(Level.WARNING, "Unable to query database for user with hostmask: " + hostmask, e);
            return null;
        }
    }

    public Integer getId() {
        return id;
    }

    public String getHostmask() {
        return hostmask;
    }

    public void setHostmask(String hostmask) {
        this.hostmask = hostmask;
    }

    public byte getLevel() {
        return level;
    }

    public void setLevel(byte level) {
        this.level = level;
    }

    public boolean isAutoOp() {
        return autoOp;
    }

    public void setAutoOp(boolean autoOp) {
        this.autoOp = autoOp;
    }

    public boolean isAutoVoice() {
        return autoVoice;
    }

    public void setAutoVoice(boolean autoVoice) {
        this.autoVoice = autoVoice;
    }

    public boolean save() {
        Connection connection = DatabaseEngine.getInstance().getConnection();

        try {
            PreparedStatement stmt;

            if (id == null) {
                stmt = connection.prepareStatement("INSERT INTO users (hostmask, level, autoOp, autoVoice) VALUES (?, ?, ?, ?)");
                stmt.setString(1, hostmask);
                stmt.setInt(2, level);
                stmt.setBoolean(3, autoOp);
                stmt.setBoolean(4, autoVoice);
            } else {
                stmt = connection.prepareStatement("UPDATE users SET hostmask = ?, level = ?, autoOp = ?, autoVoice = ? WHERE id = ?");
                stmt.setString(1, hostmask);
                stmt.setInt(2, level);
                stmt.setBoolean(3, autoOp);
                stmt.setBoolean(4, autoVoice);
                stmt.setInt(5, id);
            }

            stmt.executeUpdate();
            stmt.close();

            return true;
        } catch (SQLException e) {
            logger.log(Level.WARNING, "Unable to save user to database.", e);

            return false;
        }
    }
}
