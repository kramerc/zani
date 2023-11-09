package sh.cocaine.sexo;

import org.pircbotx.Configuration;
import org.pircbotx.PircBotX;

public class Sexo {
    public static void main(String[] args) throws Exception {
        //Configure what we want our bot to do
        Configuration configuration = new Configuration.Builder()
                .setName("oxes") //Set the nick of the bot.
                .setRealName("oxes")
                .setLogin("oxes")
                .addServer("irc.efnet.org") //Join efnet
                .addAutoJoinChannel("#camp", "truetocaesar") //Join camp
                .addListener(new Listener()) //Add our listener that will be called on Events
                .buildConfiguration();

        //Create our bot with the configuration
        PircBotX bot = new PircBotX(configuration);
        //Connect to the server
        bot.startBot();
    }
}
