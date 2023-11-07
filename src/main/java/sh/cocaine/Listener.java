package sh.cocaine;

import org.pircbotx.Configuration;
import org.pircbotx.PircBotX;
import org.pircbotx.hooks.ListenerAdapter;
import org.pircbotx.hooks.events.JoinEvent;
import org.pircbotx.hooks.events.MessageEvent;

import java.util.HashMap;


public class Listener extends ListenerAdapter {

    HashMap userList = new HashMap<String, User>() {{
        put("whale!~whale@snow.cocaine.sh", new User("whale!~whale@snow.cocaine.sh", (byte) 3, true));
    }};

    @Override
    public void onJoin(JoinEvent event) {
        System.out.println(event.getUserHostmask().getHostmask());
        if (isAutoOp(event.getUserHostmask().getHostmask())) {
            event.getChannel().send().setMode("+o " + event.getUser().getNick());
            System.out.println("Gave " + event.getUser().getNick() + " op");
        }
    }

    @Override
    public void onMessage(MessageEvent event) {
        if (event.getMessage().startsWith("!")) {
            String[] args = event.getMessage().split(" ");
            if (args[0].equals("!op") && isLevelOp(event.getUserHostmask().getHostmask())) {
                if (args.length == 2) {
                    event.getChannel().send().setMode("+o " + args[1]);
                    System.out.println("Gave " + args[1] + " op");
                } else {
                    event.getChannel().send().message("Usage: !op <nick>");
                }
            }
            if (args[0].equals("!addop") && isLevelAdmin(event.getUserHostmask().getHostmask())) {
                if (args.length == 2) {
                    event.getChannel().send().message("Added " + args[1] + " to the op list");
                    System.out.println("Added " + args[1] + " to the op list");
                    // print out userList entries
                    for (Object key : userList.keySet()) {
                        System.out.println(key + " : " + userList.get(key));
                    }
                } else {
                    event.getChannel().send().message("Usage: !addop <nick>");
                }
            }
        }
    }

    public byte getLevel(String hostmask) {
        if (userList.containsKey(hostmask)) {
            return ((User) userList.get(hostmask)).level;
        } else {
            return 0;
        }
    }

    public boolean isAutoOp(String hostmask) {
        if (userList.containsKey(hostmask)) {
            return ((User) userList.get(hostmask)).autoOp;
        } else {
            return false;
        }
    }

    public boolean isLevelOp(String hostmask) {
        if (userList.containsKey(hostmask)) {
            return ((User) userList.get(hostmask)).level > 1;
        } else {
            return false;
        }
    }
    public boolean isLevelAdmin(String hostmask) {
        if (userList.containsKey(hostmask)) {
            return ((User) userList.get(hostmask)).level > 2;
        } else {
            return false;
        }
    }

    public static void main(String[] args) throws Exception {
        //Configure what we want our bot to do
        Configuration configuration = new Configuration.Builder()
                .setName("sexo") //Set the nick of the bot.
                .setRealName("sexo")
                .setLogin("sexo")
                .addServer("irc.mzima.net") //Join efnet
                .addAutoJoinChannel("#camp", "truetocaesar") //Join camp
                .addListener(new Listener()) //Add our listener that will be called on Events
                .buildConfiguration();

        //Create our bot with the configuration
        PircBotX bot = new PircBotX(configuration);
        //Connect to the server
        bot.startBot();
    }
}