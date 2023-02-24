package ibc.ics25.handler;

import score.annotation.External;

public class IBCHandler {

    public IBCHandler() {}

    @External(readonly = true)
    public String name() {
        return "ICON IBC Handler";
    }
    
}
