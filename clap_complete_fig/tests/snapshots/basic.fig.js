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
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: "-c",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      options: [
        {
          name: "-c",
        },
      ],
      args: {
        name: "subcommand",
        isOptional: true,
      },
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: "-c",
    },
    {
      name: "-v",
    },
  ],
};

export default completion;
