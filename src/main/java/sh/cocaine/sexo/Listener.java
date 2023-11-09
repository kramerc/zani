package sh.cocaine.sexo;

import org.pircbotx.hooks.ListenerAdapter;
import org.pircbotx.hooks.events.JoinEvent;
import org.pircbotx.hooks.events.MessageEvent;
import sh.cocaine.sexo.user.User;
import sh.cocaine.sexo.user.UserLevel;

import java.util.HashMap;
import java.util.Objects;
import java.util.logging.Logger;


public class Listener extends ListenerAdapter {

    private static final Logger logger = Logger.getLogger(Listener.class.getName());

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
        String eventHostmask = filteredHostmask(event.getUserHostmask().getHostmask());

        if (event.getMessage().startsWith("!")) {
            String[] args = event.getMessage().split(" ");

            if (args[0].equals("!op") && isLevelOp(eventHostmask)) {
                if (args.length == 1) {
                    event.getChannel().send().setMode("+o " + Objects.requireNonNull(event.getUser()).getNick());
                    logger.info("Gave " + event.getUser().getNick() + " op");
                }
                if (args.length == 2) {
                    event.getChannel().send().setMode("+o " + args[1]);
                    logger.info("Gave " + args[1] + " op on behalf of " + Objects.requireNonNull(event.getUser()).getNick());
                } else {
                    event.getChannel().send().message("Usage: !op <nick>");
                }
            }

            if (args[0].equals("!addop") && isLevelAdmin(eventHostmask)) {
                if (args.length == 2) {
                    // Find the hostmask of the target user
                    String hostmask = "";
                    for (org.pircbotx.User user : event.getChannel().getUsers()) {
                        if (user.getNick().equalsIgnoreCase(args[1])) {
                            // Found target user
                            hostmask = user.getHostmask();
                            break;
                        }
                    }

                    // TODO: Figure out how to get the hostmask of a user if it is missing
                    if (!hostmask.contains("!")) {
                        event.getChannel().send().message("The bot is missing the hostmask for user " + args[1]);
                        return;
                    }

                    hostmask = filteredHostmask(hostmask);

                    User user = User.findByHostmask(hostmask);
                    if (user != null) {
                        user.setLevel(UserLevel.OP.getLevel());
                        user.setAutoOp(true);
                    } else {
                        user = new User(hostmask, UserLevel.OP.getLevel(), true, false);
                    }

                    if (user.save()) {
                        event.getChannel().send().message("Added " + args[1] + " to the op list");
                        logger.info("Added " + args[1] + " to the op list");
                        event.getChannel().send().setMode("+o " + args[1]);
                    } else {
                        event.getChannel().send().message("Failed to add " + args[1] + " to the op list");
                        logger.warning("Failed to add " + args[1] + " to the op list");
                    }
                } else {
                    event.getChannel().send().message("Usage: !addop <nick>");
                }
            }
        }
    }

    private String filteredHostmask(String hostmask) {
        if (!hostmask.contains("!")) {
            return hostmask;
        }

        return hostmask.replaceAll("~", "").split("!")[1].toLowerCase();
    }

    private boolean isAutoOp(String filteredHostmask) {
        User user = User.findByHostmask(filteredHostmask);
        if (user != null) {
            return user.isAutoOp();
        } else {
            return false;
        }
    }
    private boolean isAutoVoice(String filteredHostmask) {
        User user = User.findByHostmask(filteredHostmask);
        if (user != null) {
            return user.isAutoVoice();
        } else {
            return false;
        }
    }

    private boolean isLevelOp(String filteredHostmask) {
        User user = User.findByHostmask(filteredHostmask);
        if (user != null) {
            return user.getLevel() >= UserLevel.OP.getLevel();
        } else {
            return false;
        }
    }

    private boolean isLevelAdmin(String filteredHostmask) {
        User user = User.findByHostmask(filteredHostmask);
        if (user != null) {
            return user.getLevel() >= UserLevel.ADMIN.getLevel();
        } else {
            return false;
        }
    }

}
