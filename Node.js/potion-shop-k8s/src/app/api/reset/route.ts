import { NextResponse } from 'next/server';
import { shopStore } from '@/lib/store';

export async function POST() {
  const stats = shopStore.reset();

  return NextResponse.json({
    success: true,
    message: 'Shop statistics have been reset',
    stats,
  });
}
