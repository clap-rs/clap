const completion: Fig.Spec = {
  name: "my-app",
  description: "",
  subcommands: [
    {
      name: "cmd-single-quotes",
      description: "Can be 'always', 'auto', or 'never'",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "cmd-double-quotes",
      description: "Can be /"always/", /"auto/", or /"never/"",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "cmd-backticks",
      description: "For more information see `echo test`",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "cmd-backslash",
      description: "Avoid '//n'",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "cmd-brackets",
      description: "List packages [filter]",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "cmd-expansions",
      description: "Execute the shell command with $SHELL",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      subcommands: [
        {
          name: "cmd-single-quotes",
          description: "Can be 'always', 'auto', or 'never'",
        },
        {
          name: "cmd-double-quotes",
          description: "Can be /"always/", /"auto/", or /"never/"",
        },
        {
          name: "cmd-backticks",
          description: "For more information see `echo test`",
        },
        {
          name: "cmd-backslash",
          description: "Avoid '//n'",
        },
        {
          name: "cmd-brackets",
          description: "List packages [filter]",
        },
        {
          name: "cmd-expansions",
          description: "Execute the shell command with $SHELL",
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
        },
      ],
    },
  ],
  options: [
    {
      name: "--single-quotes",
      description: "Can be 'always', 'auto', or 'never'",
    },
    {
      name: "--double-quotes",
      description: "Can be /"always/", /"auto/", or /"never/"",
    },
    {
      name: "--backticks",
      description: "For more information see `echo test`",
    },
    {
      name: "--backslash",
      description: "Avoid '//n'",
    },
    {
      name: "--brackets",
      description: "List packages [filter]",
    },
    {
      name: "--expansions",
      description: "Execute the shell command with $SHELL",
    },
    {
      name: ["-h", "--help"],
      description: "Print help",
    },
    {
      name: ["-V", "--version"],
      description: "Print version",
    },
  ],
};

export default completion;
