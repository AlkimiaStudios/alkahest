#include <alkahest/util/allocator.h>

namespace Alkahest
{
    HeapAllocator::HeapAllocator(std::uint32_t size)
        : m_size(Memory::align(size))
    {
        AKST_ENG_ASSERT(size > 0, "Allocated size must be greater than zero!");

        m_heap = std::malloc(Memory::align(size));
    }

    HeapAllocator::~HeapAllocator()
    {
        std::free(m_heap);
    }

    void *HeapAllocator::findAvailableSlot(std::uint32_t size)
    {
        // Empty heap edge case
        if (m_allocations.size() == 0)
        {
            return m_heap;
        }

        // Iterate through allocations, comparing end of one chunk to beginning of next
        std::map<void*, Memory::chunk>::iterator i = m_allocations.begin();
        while (i != m_allocations.end())
        {
            Memory::chunk& c = i->second;
            // If c is the last item then d would be map.end() here
            i++;
            Memory::chunk d;
            if (i == m_allocations.end())
            {
                // ...so we set the "next chunk start" to the first byte
                // beyond the heap to avoid allocating invalid mem
                d.start = static_cast<char*>(m_heap) + m_size;
            }
            else {
                d = i->second;
            }

            if (static_cast<std::uint32_t>(static_cast<char*>(d.start) - static_cast<char*>(c.end)) >= size)
            {
                // Return the next available memory location since we have
                // found a suitable mem slot
                return static_cast<void*>(static_cast<char*>(c.end) + 1);
            }
        }

        // This will cause a segfault for OOM
        return nullptr;
    }

    void *HeapAllocator::alloc(std::uint32_t size)
    {
        // Use align() to keep allocations aligned to buffer size
        void *start = findAvailableSlot(Memory::align(size));

        // Create a chunk to log the allocation
        Memory::chunk c;
        c.start = start;
        c.size = Memory::align(size);
        c.end = static_cast<void*>(static_cast<char*>(c.start) + c.size);
        m_allocations.insert(std::pair<void*,Memory::chunk>(start, c));

        logTrace("Allocated {} bytes from heap", Memory::align(size));
        return start;
    }

    void HeapAllocator::free(void *p)
    {
        std::map<void*, Memory::chunk>::iterator c = m_allocations.find(p);
        if (c != m_allocations.end())
        {
            m_allocations.erase(p);
            return;
        }
    }

    StackAllocator::StackAllocator(std::uint32_t size)
        : m_size(Memory::align(size))
    {
        AKST_ENG_ASSERT(size > 0, "Allocated size must be greater than zero!");

        m_stack = std::malloc(Memory::align(size));
        m_topMarker = 0;
    }

    StackAllocator::~StackAllocator()
    {
        std::free(m_stack);
    }

    void StackAllocator::clear()
    {
        m_topMarker = 0;
    }

    void *StackAllocator::alloc(std::uint32_t size)
    {
        AKST_ENG_ASSERT(m_topMarker + Memory::align(size) < m_size, \
            "Insufficient space to allocate bytes!");

        void *chunk = static_cast<void*>(static_cast<char*>(m_stack) \
                + m_topMarker);
        m_topMarker += Memory::align(size);
        return chunk;
    }

    Marker StackAllocator::getMarker()
    {
        return m_topMarker;
    }

    void StackAllocator::freeToMarker(Marker m)
    {
        AKST_ENG_ASSERT(m >= 0, "Cannot free memory to a negative address!");
        AKST_ENG_ASSERT(m < m_topMarker, "m must be below the current top of the stack!");

        m_topMarker = m;
    }
}
