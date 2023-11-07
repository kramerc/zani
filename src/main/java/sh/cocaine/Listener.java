package sh.cocaine;

import org.pircbotx.Configuration;
import org.pircbotx.PircBotX;
import org.pircbotx.hooks.ListenerAdapter;
import org.pircbotx.hooks.events.JoinEvent;
import org.pircbotx.hooks.events.MessageEvent;

import java.util.HashMap;


public class Listener extends ListenerAdapter {

    HashMap userList = new HashMap<String, User>() {{
        put("whale@snow.cocaine.sh", new User("whale@snow.cocaine.sh", (byte) 3, true));
        put("horse@pony.equus.sh", new User("horse@pony.equus.sh", (byte) 1, true));
        put("no@50.47.219.16", new User("no@50.47.219.16", (byte) 1, true));
        put("kr@m3r.sh", new User("kr@m3r.sh", (byte) 1, true));
    }};

    @Override
    public void onJoin(JoinEvent event) {
        System.out.println(event.getUserHostmask().getHostmask());
        if (isAutoOp(event.getUserHostmask().getHostmask())) {
            event.getChannel().send().setMode("+o " + event.getUser().getNick());
            System.out.println("Gave " + event.getUser().getNick() + " auto op");
        }
    }

    @Override
    public void onMessage(MessageEvent event) {

        String eventHostmask = event.getUserHostmask().getHostmask();

        if (event.getMessage().startsWith("!")) {
            String[] args = event.getMessage().split(" ");
            if (args[0].equals("!op") && isLevelOp(eventHostmask)) {
                if (args.length == 1) {
                    event.getChannel().send().setMode("+o " + event.getUser().getNick());
                    System.out.println("Gave " + event.getUser().getNick() + " op");
                }
                if (args.length == 2) {
                    event.getChannel().send().setMode("+o " + args[1]);
                    System.out.println("Gave " + args[1] + " op on behalf of " + event.getUser().getNick());
                } else {
                    event.getChannel().send().message("Usage: !op <nick>");
                }
            }
            if (args[0].equals("!addop") && isLevelAdmin(eventHostmask)) {
                if (args.length == 2) {
                    // create a hashmap of the users in channel and their hostmasks
                    HashMap<String, String> channelUsers = new HashMap<String, String>();
                    for (Object user : event.getChannel().getUsers()) {
                        channelUsers.put(((org.pircbotx.User) user).getNick().toLowerCase(), ((org.pircbotx.User) user).getHostmask());
                    }

                    // if userList doesn't contain the hostmask, add it
                    if (!userList.containsKey(filteredHostmask(channelUsers.get(args[1].toLowerCase())))) {
                        userList.put(filteredHostmask(channelUsers.get(args[1].toLowerCase())),
                                new User(filteredHostmask(channelUsers.get(args[1].toLowerCase())), (byte) 1, true));

                        event.getChannel().send().message("Added " + args[1] + " to the op list");
                        System.out.println("Added " + args[1] + " to the op list");
                    } else {
                        System.out.println("User " + args[1] + " already exists in the list.");
                    }

                    // print out userList entries
                    for (Object key : userList.keySet()) {
                        System.out.println(key + " : " + userList.get(key).toString());
                    }
                } else {
                    event.getChannel().send().message("Usage: !addop <nick>");
                }
            }
        }
    }

    public String filteredHostmask(String hostmask) {
        return hostmask.replaceAll("~", "").split("!")[1].toLowerCase();
    }

    public boolean isAutoOp(String hostmask) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            return ((User) userList.get(filteredHostmask(hostmask))).autoOp;
        } else {
            return false;
        }
    }

    public boolean isLevelOp(String hostmask) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            return ((User) userList.get(filteredHostmask(hostmask))).level >= 1;
        } else {
            return false;
        }
    }
    public boolean isLevelAdmin(String hostmask) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            return ((User) userList.get(filteredHostmask(hostmask))).level >= 2;
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
//                .addAutoJoinChannel("#camp", "truetocaesar") //Join camp
                .addAutoJoinChannel("#drenched") //Join drenched
                .addListener(new Listener()) //Add our listener that will be called on Events
                .buildConfiguration();

        //Create our bot with the configuration
        PircBotX bot = new PircBotX(configuration);
        //Connect to the server
        bot.startBot();
    }
}