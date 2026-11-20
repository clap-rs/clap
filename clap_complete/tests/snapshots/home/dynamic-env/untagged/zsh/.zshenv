fpath=($fpath $ZDOTDIR/zsh)
autoload -U +X compinit && compinit -u # bypass compaudit security checking
precmd_functions=""  # avoid the prompt being overwritten
PS1='%% '
PROMPT='%% '
