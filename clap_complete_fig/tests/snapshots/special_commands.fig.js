const completion: Fig.Spec = {
  name: "my-app",
  description: "Tests completions",
  subcommands: [
    {
      name: "test",
      description: "tests things",
      options: [
        {
          name: "--case",
          description: "the case to test",
          isRepeatable: true,
          args: {
            name: "case",
            isOptional: true,
          },
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
    },
    {
      name: "some_cmd",
      description: "tests other things",
      options: [
        {
          name: "--config",
          description: "the other case to test",
          hidden: true,
          isRepeatable: true,
          requiresEquals: true,
          args: {
            name: "config",
            isOptional: true,
          },
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
      args: {
        name: "path",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["some-cmd-with-hyphens", "hyphen"],
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
        {
          name: ["-V", "--version"],
          description: "Print version",
        },
      ],
    },
    {
      name: "some-hidden-cmd",
      hidden: true,
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
        {
          name: ["-V", "--version"],
          description: "Print version",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      subcommands: [
        {
          name: "test",
          description: "tests things",
        },
        {
          name: "some_cmd",
          description: "tests other things",
        },
        {
          name: "some-cmd-with-hyphens",
        },
        {
          name: "some-hidden-cmd",
          hidden: true,
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
      name: ["-c", "-C", "--config", "--conf"],
      description: "some config file",
      isRepeatable: true,
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
  args: [
    {
      name: "file",
      isOptional: true,
      template: "filepaths",
    },
    {
      name: "choice",
      isOptional: true,
      suggestions: [
        "first",
        "second",
      ],
    },
  ]
};

export default completion;
