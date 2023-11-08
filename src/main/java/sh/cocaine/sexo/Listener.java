package sh.cocaine.sexo;

import org.pircbotx.Configuration;
import org.pircbotx.PircBotX;
import org.pircbotx.User;
import org.pircbotx.hooks.ListenerAdapter;
import org.pircbotx.hooks.events.JoinEvent;
import org.pircbotx.hooks.events.MessageEvent;

import java.util.HashMap;
import java.util.Objects;


public class Listener extends ListenerAdapter {

    HashMap<String, BotUser> userList = new HashMap<>() {{
        put("whale@snow.cocaine.sh", new BotUser("whale@snow.cocaine.sh", (byte) 2, true, false));
        put("horse@pony.equus.sh", new BotUser("horse@pony.equus.sh", (byte) 2, true, false));
        put("no@50.47.219.16", new BotUser("no@50.47.219.16", (byte) 2, true, false));
        put("kr@m3r.sh", new BotUser("kr@m3r.sh", (byte) 2, true, false));
        put("sigmakitty@hammond.expi.pl", new BotUser("sigmakitty@hammond.expi.pl", (byte) 2, true, false));
        put("dolphin@static.191.75.78.5.clients.your-server.de", new BotUser("dolphin@static.191.75.78.5.clients.your-server.de", (byte) 0, false, true));
    }};

    @Override
    public void onJoin(JoinEvent event) {
        String eventHostmask = event.getUserHostmask().getHostmask();

        if (isAutoOp(eventHostmask)) {
            event.getChannel().send().setMode("+o " + Objects.requireNonNull(event.getUser()).getNick());
            System.out.println("Gave " + event.getUser().getNick() + " auto op");
        }

        if (isAutoVoice(eventHostmask)) {
            event.getChannel().send().setMode("+v " + Objects.requireNonNull(event.getUser()).getNick());
            System.out.println("Gave " + event.getUser().getNick() + " auto voice");
        }
    }

    @Override
    public void onMessage(MessageEvent event) {
        String eventHostmask = event.getUserHostmask().getHostmask();

        if (event.getMessage().startsWith("!")) {
            String[] args = event.getMessage().split(" ");

            // create a hashmap of the users in channel and their hostmasks
            HashMap<String, String> channelUsers = new HashMap<String, String>();
            for (User user : event.getChannel().getUsers()) {
                channelUsers.put(user.getNick().toLowerCase(), user.getHostmask());
            }

            if (args[0].equals("!op") && isLevelOp(eventHostmask)) {
                if (args.length == 1) {
                    event.getChannel().send().setMode("+o " + Objects.requireNonNull(event.getUser()).getNick());
                    System.out.println("Gave " + event.getUser().getNick() + " op");
                }
                if (args.length == 2) {
                    event.getChannel().send().setMode("+o " + args[1]);
                    System.out.println("Gave " + args[1] + " op on behalf of " + Objects.requireNonNull(event.getUser()).getNick());
                } else {
                    event.getChannel().send().message("Usage: !op <nick>");
                }
            }

            if(args[0].equals("!voice") && isLevelOp(eventHostmask)) {
                if (args.length == 1) {
                    event.getChannel().send().setMode("+v " + Objects.requireNonNull(event.getUser()).getNick());
                    System.out.println("Gave " + event.getUser().getNick() + " voice");
                }
                if (args.length == 2) {
                    event.getChannel().send().setMode("+v " + args[1]);
                    System.out.println("Gave " + args[1] + " voice on behalf of " + Objects.requireNonNull(event.getUser()).getNick());
                } else {
                    event.getChannel().send().message("Usage: !voice <nick>");
                }
            }

            if (args[0].equals("!addop") && isLevelAdmin(eventHostmask)) {
                if (args.length == 2) {

                    // if userList doesn't contain the hostmask, add it
                    if (!doesBotUserExist(filteredHostmask(channelUsers.get(args[1].toLowerCase())))) {
                        addBotUser(filteredHostmask(channelUsers.get(args[1].toLowerCase())), (byte) 1, true, false);

                        event.getChannel().send().message("Added " + args[1] + " to the op list");
                        System.out.println("Added " + args[1] + " to the op list");
                    } else {
                        System.out.println(args[1] + " already exists in the op list.");
                        if(isLevelUser(filteredHostmask(channelUsers.get(args[1].toLowerCase())))) {
                            setUserLevel(filteredHostmask(channelUsers.get(args[1].toLowerCase())), (byte) 1);
                        }
                        setAutoOp(filteredHostmask(channelUsers.get(args[1].toLowerCase())), true);
                    }
                } else {
                    event.getChannel().send().message("Usage: !addop <nick>");
                }
            }

            if (args[0].equals("!addvoice") && isLevelOp(eventHostmask)) {
                if (args.length == 2) {

                    // if userList doesn't contain the hostmask, add it
                    if (!doesBotUserExist(filteredHostmask(channelUsers.get(args[1].toLowerCase())))) {
                        addBotUser(filteredHostmask(channelUsers.get(args[1].toLowerCase())), (byte) 0, false, true);

                        event.getChannel().send().message("Added " + args[1] + " to the voice list");
                        System.out.println("Added " + args[1] + " to the voice list");
                    } else {
                        System.out.println(args[1] + " already exists in the voice list.");
                        if(isLevelUser(filteredHostmask(channelUsers.get(args[1].toLowerCase())))) {
                            setUserLevel(filteredHostmask(channelUsers.get(args[1].toLowerCase())), (byte) 0);
                        }
                        setAutoVoice(filteredHostmask(channelUsers.get(args[1].toLowerCase())), true);
                    }
                } else {
                    event.getChannel().send().message("Usage: !addvoice <nick>");
                }
            }

            if (args[0].equals("!removeBotUser") && isLevelAdmin(eventHostmask)) {
                if (args.length == 2) {
                    if (doesBotUserExist(filteredHostmask(channelUsers.get(args[1].toLowerCase())))) {
                        userList.remove(filteredHostmask(channelUsers.get(args[1].toLowerCase())));

                        event.getChannel().send().message("Removed " + args[1] + " from the bot user list");
                        System.out.println("Removed " + args[1] + " from the bot user list");
                    } else {
                        event.getChannel().send().message(args[1] + " does not exist in the bot user list");
                        System.out.println(args[1] + " does not exist in the bot user list");
                    }
                } else {
                    event.getChannel().send().message("Usage: !removeBotUser <nick>");
                }
            }
        }
    }

    public String filteredHostmask(String hostmask) {
        return hostmask.replaceAll("~", "").split("!")[1].toLowerCase();
    }

    public boolean isAutoOp(String hostmask) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            return (userList.get(filteredHostmask(hostmask))).autoOp;
        } else {
            return false;
        }
    }
    public boolean isAutoVoice(String hostmask) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            return (userList.get(filteredHostmask(hostmask))).autoVoice;
        } else {
            return false;
        }
    }

    public boolean isLevelUser(String hostmask) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            return (userList.get(filteredHostmask(hostmask))).level == 0;
        } else {
            return false;
        }
    }

    public boolean isLevelOp(String hostmask) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            return (userList.get(filteredHostmask(hostmask))).level >= 1;
        } else {
            return false;
        }
    }
    public boolean isLevelAdmin(String hostmask) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            return (userList.get(filteredHostmask(hostmask))).level >= 2;
        } else {
            return false;
        }
    }

    public boolean doesBotUserExist(String hostmask) {
        return userList.containsKey(filteredHostmask(hostmask));
    }

    public void addBotUser(String hostmask, byte level, boolean autoOp, boolean autoVoice) {
        userList.put(hostmask, new BotUser(hostmask, level, autoOp, autoVoice));
    }

    public void setUserLevel(String hostmask, byte level) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            userList.get(filteredHostmask(hostmask)).level = level;
        }
    }

    public void setAutoOp(String hostmask, boolean autoOp) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            userList.get(filteredHostmask(hostmask)).autoOp = autoOp;
        }
    }

    public void setAutoVoice(String hostmask, boolean autoVoice) {
        if (userList.containsKey(filteredHostmask(hostmask))) {
            userList.get(filteredHostmask(hostmask)).autoVoice = autoVoice;
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