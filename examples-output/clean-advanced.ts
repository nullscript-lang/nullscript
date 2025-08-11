// Clean advanced TypeScript features in NullScript

// Basic types && interfaces
interface User {
  id: string;
  name: string;
  age: number;
  isActive?: boolean;
}

type UserID = string;
type Status = 'active' | 'inactive';

// Generic interface
interface ApiResponse<T> {
  data: T;
  status: number;
  message?: string;
}

// Enum
enum UserRole {
  Admin = 'admin',
  User = 'user',
  Guest = 'guest'
}

// Class with generics
class DataStore<T extends { id: string }> {
  private items: Map<string, T> = new Map();

  add(item: T): void {
    this.items.set(item.id, item);
  }

  get(id: string): T | undefined {
    return this.items.get(id) ?? undefined;
  }

  getAll(): T[] {
    return Array.from(this.items.values());
  }
}

// Functions
function createUser(name: string, age: number): User {
  return {
    id: Math.random().toString(36),
    name,
    age,
    isActive: true
  };
}

async function fetchUser(id: string): Promise<User | null> {
  try {
    const response = await fetch(`/api/users/${id}`);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('Failed to fetch user:', error);
    return null;
  }
}

function greetUser(user: User): string {
  if (user.isActive) {
    return `Hello, ${user.name}! Welcome back.`;
  } else {
    return `Hello, ${user.name}. Please activate your account.`;
  }
}

// Working with the data store
const userStore = new DataStore<User>();

const sampleUsers: User[] = [
  createUser("Alice", 25),
  createUser("Bob", 30),
  createUser("Charlie", 28)
];

// Add users to store
for (const user of sampleUsers) {
  userStore.add(user);
}

// Print all users
for (const user of userStore.getAll()) {
  console.log(greetUser(user));
}

export { User, DataStore, createUser, greetUser };
