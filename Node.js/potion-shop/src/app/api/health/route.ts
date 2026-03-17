import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    status: 'healthy',
    shop: 'open',
    potions_available: true,
  });
}
