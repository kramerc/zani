package af.camp.sexo;

import org.pircbotx.Configuration;
import org.pircbotx.PircBotX;
import af.camp.sexo.db.DatabaseEngine;

import java.util.logging.Logger;

public class Sexo {

    private static final Logger logger = Logger.getLogger(Sexo.class.getName());

    private static PircBotX bot;

    private static YamlConfig yamlConfig;

    public static void main(String[] args) throws Exception {
        logger.info("Starting Sexo...");

        yamlConfig = YamlConfig.load();

        // Set up the database.
        DatabaseEngine db = DatabaseEngine.getInstance();
        db.connect();
        db.prepare();
        db.seed();

        //Configure what we want our bot to do
        Configuration.Builder configBuilder = new Configuration.Builder()
                .setAutoReconnect(true)
                .setOnJoinWhoEnabled(true)
                .setName(yamlConfig.getNick())
                .setRealName(yamlConfig.getRealName())
                .setLogin(yamlConfig.getLogin())
                .addServer(yamlConfig.getServer(), yamlConfig.getPort())
                .addListener(new Listener()); //Add our listener that will be called on Events

        // Add channels defined in the configuration.
        for (String channelLine : yamlConfig.getChannels()) {
            String[] bits = channelLine.split(" ", 2);
            String channel = bits[0];
            if (bits.length > 1) {
                String key = bits[1];
                configBuilder.addAutoJoinChannel(channel, key);
            } else {
                configBuilder.addAutoJoinChannel(channel);
            }
        }

        Configuration configuration = configBuilder.buildConfiguration();

        bot = new PircBotX(configuration);
        bot.startBot();
    }


    /**
     * Gets the bot instance.
     *
     * @return The bot instance.
     */
    public static PircBotX getBot() {
        return bot;
    }

    /**
     * Gets the {@link YamlConfig} instance.
     *
     * @return The {@code YamlConfig} instance.
     */
    public static YamlConfig getYamlConfig() {
        return yamlConfig;
    }
}
