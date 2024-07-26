package af.camp.sexo;

import org.yaml.snakeyaml.Yaml;

import java.io.*;
import java.util.List;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 * Handles the bot's Yaml configuration.
 */
public class YamlConfig {

    private static final Logger logger = Logger.getLogger(YamlConfig.class.getName());

    private static final String CONFIG_FILE = "sexo.yml";

    private String dbUrl;

    private String nick;

    private String realName;

    private String login;

    private String server;

    private int port;

    private List<String> channels;


    /**
     * Loads the configuration.
     */
    public static YamlConfig load() throws FileNotFoundException {
        Yaml yaml = new Yaml();

        try {
            return yaml.loadAs(new FileReader(CONFIG_FILE), YamlConfig.class);
        } catch (FileNotFoundException e) {
            logger.log(Level.SEVERE, "Unable to load configuration.", e);
            throw e;
        }
    }

    /**
     * Saves the configuration.
     */
    public void save() throws FileNotFoundException {
        Yaml yaml = new Yaml();

        try {
            yaml.dump(this, new PrintWriter(CONFIG_FILE));
        } catch (FileNotFoundException e) {
            logger.log(Level.SEVERE, "Unable to save configuration.", e);
            throw e;
        }
    }

    public String getDbUrl() {
        return dbUrl;
    }

    public void setDbUrl(String dbUrl) {
        this.dbUrl = dbUrl;
    }

    public String getNick() {
        return nick;
    }

    public void setNick(String nick) {
        this.nick = nick;
    }

    public String getRealName() {
        return realName;
    }

    public void setRealName(String realName) {
        this.realName = realName;
    }

    public String getLogin() {
        return login;
    }

    public void setLogin(String login) {
        this.login = login;
    }

    public String getServer() {
        return server;
    }

    public void setServer(String server) {
        this.server = server;
    }

    public int getPort() {
        return port;
    }

    public void setPort(int port) {
        this.port = port;
    }

    public List<String> getChannels() {
        return channels;
    }

    public void setChannels(List<String> channels) {
        this.channels = channels;
    }
}
