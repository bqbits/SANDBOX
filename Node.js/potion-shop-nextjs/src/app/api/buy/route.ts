import { NextRequest, NextResponse } from 'next/server';
import { shopStore } from '@/lib/store';
import { getRandomMessage } from '@/lib/messages';

export async function POST(request: NextRequest) {
  // Update shop statistics (Red potion costs 50 gold)
  const stats = shopStore.incrementSales(50);

  // Get a random fun message
  const message = getRandomMessage();

  return NextResponse.json({
    success: true,
    message,
    stats,
  });
}
