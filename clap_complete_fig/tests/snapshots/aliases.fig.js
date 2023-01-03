const completion: Fig.Spec = {
  name: "my-app",
  description: "testing bash completions",
  options: [
    {
      name: ["-o", "-O", "--option", "--opt"],
      description: "cmd option",
      isRepeatable: true,
      args: {
        name: "option",
        isOptional: true,
      },
    },
    {
      name: ["-f", "-F", "--flag", "--flg"],
      description: "cmd flag",
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
    name: "positional",
    isOptional: true,
  },
};

export default completion;
