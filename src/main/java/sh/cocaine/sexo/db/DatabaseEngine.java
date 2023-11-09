package sh.cocaine.sexo.db;

import sh.cocaine.sexo.Sexo;
import sh.cocaine.sexo.user.User;
import sh.cocaine.sexo.user.UserLevel;

import java.sql.*;
import java.util.ArrayList;
import java.util.List;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 * A database engine that utilizes SQLite.
 */
public class DatabaseEngine {

    private static final Logger logger = Logger.getLogger(DatabaseEngine.class.getName());

    private static DatabaseEngine instance;

    private Connection connection;

    public DatabaseEngine() {
        try {
            Class.forName("org.sqlite.JDBC");
        } catch (ClassNotFoundException e) {
            logger.log(Level.SEVERE, "Unable to initialize load database engine.", e);
        }
    }

    /**
     * Creates a connection to the database.
     *
     * @throws SQLException If unable to connect to the database.
     */
    public synchronized void connect() {
        try {
            connection = DriverManager.getConnection(Sexo.getYamlConfig().getDbUrl());
        } catch (SQLException e) {
            logger.log(Level.SEVERE,
                    "Unable to create a connection to the database.", e);
        }
    }

    /**
     * Prepares the database with default tables if they do not already exist.
     */
    public synchronized void prepare() {
        try {
            Statement stmt = connection.createStatement();
            stmt.executeUpdate(
                    "CREATE TABLE IF NOT EXISTS users (" +
                            "  id INTEGER PRIMARY KEY, " +
                            "  hostmask VARCHAR(512) UNIQUE, " +
                            "  level INT(11), " +
                            "  autoOp INT(1) DEFAULT 0, " +
                            "  autoVoice INT(1) DEFAULT 0" +
                            ")"
            );

        } catch (SQLException e) {
            logger.log(Level.SEVERE, "Unable to prepare the database.", e);
        }
    }

    public synchronized void seed() {
        List<User> users = new ArrayList<>();
        users.add(new User("whale@snow.cocaine.sh", UserLevel.ADMIN.getLevel(), true, false));
        users.add(new User("horse@pony.equus.sh", UserLevel.OP.getLevel(), true, false));
        users.add(new User("no@50.47.219.16", UserLevel.OP.getLevel(), true, false));
        users.add(new User("kr@m3r.sh", UserLevel.OP.getLevel(), true, false));
        users.add(new User("sigmakitty@hammond.expi.pl", UserLevel.OP.getLevel(), true, false));
        users.add(new User("dolphin@static.191.75.78.5.clients.your-server.de", UserLevel.USER.getLevel(), false, true));

        try {
            PreparedStatement stmt = connection.prepareStatement("INSERT OR IGNORE INTO users (hostmask, level, autoOp, autoVoice) VALUES (?, ?, ?, ?)");

            for (User user : users) {
                stmt.setString(1, user.getHostmask());
                stmt.setInt(2, user.getLevel());
                stmt.setBoolean(3, user.isAutoOp());
                stmt.setBoolean(4, user.isAutoVoice());
                stmt.executeUpdate();
            }
        } catch (SQLException e) {
            logger.log(Level.SEVERE, "Unable to seed the database.", e);
        }
    }

    /**
     * Gets the connection to the database.
     *
     * @return The connection.
     */
    public Connection getConnection() {
        return connection;
    }

    /**
     * Gets the singleton instance of {@code DatabaseEngine}
     *
     * @return The singleton instance.
     */
    public static DatabaseEngine getInstance() {
        if (instance == null) {
            instance = new DatabaseEngine();
        }

        return instance;
    }

}
