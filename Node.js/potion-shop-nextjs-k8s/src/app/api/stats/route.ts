import { NextResponse } from 'next/server';
import { shopStore } from '@/lib/store';

export async function GET() {
  const stats = shopStore.getStats();

  return NextResponse.json({
    potions_sold: stats.potions_sold,
    gold_earned: stats.gold_earned,
    gold_per_potion: 50,
  });
}
