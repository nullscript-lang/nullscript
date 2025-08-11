// NullScript: Advanced TypeScript features showcase
import * as fs from 'fs/promises';

// Basic interface for examples
interface Person {
  id: string;
  name: string;
  age: number;
  isStudent?: boolean;
}

// Generic types with fun keywords
type Optional<T> = T | null;
type Result<T, E = Error> = { success: true; data: T } | { success: false; error: E };

// Advanced interface
interface ApiResponse<T> {
  readonly status: number;
  data?: T;
  errors?: string[];
}

// Enum with custom values
enum Colors {
  Red = "#ff0000",
  Green = "#00ff00",
  Blue = "#0000ff"
}

// Generic class with constraints
class DataStore<T extends { id: string }> {
  private items: Map<string, T> = new Map();

  add(item: T): void {
    this.items.set(item.id, item);
  }

  get(id: string): Optional<T> {
    return this.items.get(id) ?? null;
  }

  getAll(): T[] {
    return Array.from(this.items.values());
  }

  remove(id: string): boolean {
    return this.items.delete(id);
  }
}

// Union types && discriminated unions
type Shape =
  | { kind: 'circle'; radius: number }
  | { kind: 'rectangle'; width: number; height: number }
  | { kind: 'square'; size: number };

function calculateArea(shape: Shape): number {
  switch (shape.kind) {
    case 'circle':
      return Math.PI * shape.radius ** 2;
    case 'rectangle':
      return shape.width * shape.height;
    case 'square':
      return shape.size ** 2;
    default:
      throw new Error('Unknown shape');
  }
}

// Async/await with error handling
async function fetchUserData(userId: string): Promise<Result<Person, string>> {
  try {
    const response = await fetch(`/api/users/${userId}`);

    if (!response.ok) {
      return { success: false, error: `HTTP ${response.status}` };
    }

    const userData = await response.json();
    return { success: true, data: userData };
  } catch (error) {
    return { success: false, error: error.message };
  }
}

// Utility types && mapped types
type PartialPerson = Partial<Person>;
type RequiredKeys<T> = { [K in keyof T]-?: T[K] };

// Template literal types
type EventNames = `on${Capitalize<string>}`;
type ColorVariant = `${keyof typeof Colors}Light` | `${keyof typeof Colors}Dark`;

// Conditional types
type NonNullable<T> = T extends null | undefined ? never : T;

// Working with the data store
const personStore = new DataStore<Person>();

const samplePeople: Person[] = [
  { id: '1', name: 'Alice', age: 25, isStudent: true },
  { id: '2', name: 'Bob', age: 30, isStudent: false },
];

for (const person of samplePeople) {
  personStore.add(person);
}

// Function overloads
function processValue(value: string): string;
function processValue(value: number): number;
function processValue(value: boolean): string;
function processValue(value: any): any {
  if (typeof value === 'string') {
    return value.toUpperCase();
  } else if (typeof value === 'number') {
    return value * 2;
  } else if (typeof value === 'boolean') {
    return value ? 'yes' : 'no';
  } else {
    return value;
  }
}

// Simple logging utility (decorators can be complex)
function withLogging<T extends (...args: any[]) => any>(fn: T, name: string): T {
  return ((...args: any[]) => {
    console.log(`Calling ${name} with args:`, args);
    const result = fn(...args);
    console.log(`${name} returned:`, result);
    return result;
  }) as T;
}

class Calculator {
  add(a: number, b: number): number {
    return a + b;
  }

  multiply(a: number, b: number): number {
    return a * b;
  }
  
  getLoggedAdd() {
    return withLogging(this.add.bind(this), 'add');
  }
  
  getLoggedMultiply() {
    return withLogging(this.multiply.bind(this), 'multiply');
  }
}

// Testing our advanced features
const calc = new Calculator();
console.log('Calculator test:', calc.add(5, 3));

// Testing with logging
const loggedAdd = calc.getLoggedAdd();
console.log('Logged calculator test:', loggedAdd(5, 3));

const shapes: Shape[] = [
  { kind: 'circle', radius: 5 },
  { kind: 'rectangle', width: 4, height: 6 },
  { kind: 'square', size: 3 }
];

for (const shape of shapes) {
  console.log(`Area of ${shape.kind}:`, calculateArea(shape));
}

// Async execution
async function main() {
  const result = await fetchUserData('123');

  if (result.success) {
    console.log('User data:', result.data);
  } else {
    console.error('Failed to fetch user:', result.error);
  }
}

// Run async function
main().catch(console.error);

export { DataStore, calculateArea, Colors, Calculator };
