package ibc.icon.score.util;

import score.Context;
import score.annotation.Optional;

public class NullChecker {
    public static void requireNotNull(Object s, @Optional String msg) {
        if (msg == null) {
            msg = "cannot be null";
        }
        Context.require(s != null, msg);
    }
}
