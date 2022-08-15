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
          description: "Print help information",
        },
      ],
    },
    {
      name: "cmd-double-quotes",
      description: "Can be /"always/", /"auto/", or /"never/"",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
      ],
    },
    {
      name: "cmd-backticks",
      description: "For more information see `echo test`",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
      ],
    },
    {
      name: "cmd-backslash",
      description: "Avoid '//n'",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
      ],
    },
    {
      name: "cmd-brackets",
      description: "List packages [filter]",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
      ],
    },
    {
      name: "cmd-expansions",
      description: "Execute the shell command with $SHELL",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
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
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help information",
            },
          ],
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
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
};

export default completion;
