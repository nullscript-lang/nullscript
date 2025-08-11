// Simple NullScript example

interface User {
  name: string;
  age: number;
}

function greet(user: User): string {
  if (user.age >= 18) {
    return `Hello adult ${user.name}!`;
  } else {
    return `Hello young ${user.name}!`;
  }
}

const users: User[] = [
  { name: "Alice", age: 25 },
  { name: "Bob", age: 16 }
];

for (const user of users) {
  console.log(greet(user));
}

export { greet };
