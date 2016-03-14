#!/bin/sh

SESSION_NAME="scan_server"

tmux has-session -t ${SESSION_NAME}

if [ $? != 0 ]
then
    # Create new session
    tmux new-session -s ${SESSION_NAME} -n rust -d
    tmux split-window -h
    tmux resize-pane -R 10

    
fi

tmux attach -t ${SESSION_NAME}
