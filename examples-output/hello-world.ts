// NullScript example: Basic TypeScript functionality with fun keywords
import { readFileSync } from 'fs';

interface Person {
  name: string;
  age: number;
  isStudent?: boolean;
}

class Greeter {
  private message: string;

  constructor(greeting: string) {
    this.message = greeting;
  }

  greet(person: Person): string {
    if (person.isStudent) {
      return `${this.message}, ${person.name}! Hope your studies are going well.`;
    } else {
      return `${this.message}, ${person.name}!`;
    }
  }
}

function createPerson(name: string, age: number, isStudent = false): Person {
  return { name, age, isStudent };
}

// Main execution
const greeter = new Greeter("Hey there");

let people: Person[] = [
  createPerson("Alice", 25, true),
  createPerson("Bob", 30),
  createPerson("Charlie", 22, true)
];

for (const person of people) {
  console.log(greeter.greet(person));
}

// Error handling example
try {
  const data = readFileSync('nonexistent.txt', 'utf8');
  console.log(data);
} catch (error) {
  console.error('Oops, file ! found:', error.message);
} finally {
  console.log('Cleanup complete!');
}

export { Greeter, createPerson };
