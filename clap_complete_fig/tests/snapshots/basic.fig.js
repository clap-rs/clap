const completion: Fig.Spec = {
  name: "my-app",
  description: "",
  subcommands: [
    {
      name: "test",
      description: "Subcommand",
      options: [
        {
          name: "-d",
          isRepeatable: true,
        },
        {
          name: "-c",
        },
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
          name: "test",
          description: "Subcommand",
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
        },
      ],
      options: [
        {
          name: "-c",
        },
      ],
    },
  ],
  options: [
    {
      name: "-c",
    },
    {
      name: "-v",
      exclusiveOn: [
        "-c",
      ],
    },
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
  ],
};

export default completion;
