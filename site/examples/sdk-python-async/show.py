import asyncio

import thinkthen as tt


async def main() -> None:
    async with tt.AsyncClient() as client:
        is_refund = await client.decide(
            "Does the customer request a refund?",
            "Please refund my order.",
        )
        print(is_refund)


asyncio.run(main())
