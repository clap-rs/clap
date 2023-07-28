fpath=($fpath $ZDOTDIR/zsh)
autoload -U +X compinit && compinit
precmd_functions=""  # avoid the prompt being overwritten
PS1='%% '
PROMPT='%% '
