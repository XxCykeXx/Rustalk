// TypeScript type definitions for Rustalk P2P Chat

export interface User {
  id: string;
  email: string;
  nickname?: string;
  publicKey: string;
  isOnline: boolean;
  lastSeen: Date;
  ipAddress?: string;
  port?: number;
}

export interface Message {
  id: string;
  fromUserId: string;
  toUserId: string;
  content: string;
  timestamp: Date;
  encrypted: boolean;
  messageType: 'text' | 'file' | 'system';
}

export interface Connection {
  userId: string;
  socket: any; // WebSocket or TCP socket
  isActive: boolean;
  lastPing: Date;
  encryption: {
    publicKey: string;
    sharedSecret?: string;
  };
}

export interface Config {
  user: User;
  connections: Connection[];
  settings: {
    port: number;
    encryptionEnabled: boolean;
    theme: 'dark' | 'light';
    autoConnect: boolean;
  };
}

export interface Command {
  name: string;
  args: string[];
  description: string;
}

export interface PeerStatus {
  userId: string;
  isOnline: boolean;
  lastSeen: Date;
  responseTime?: number;
}

export interface ChatRoom {
  id: string;
  name: string;
  participants: User[];
  messages: Message[];
  isActive: boolean;
}