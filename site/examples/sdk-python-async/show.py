import asyncio

import thinkthen.aio as aio


async def main() -> None:
    is_refund = await aio.decide(
        "Does the customer request a refund?",
        "Please refund my order.",
    )
    print(is_refund)


asyncio.run(main())
