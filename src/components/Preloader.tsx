import { useEffect, useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';

export default function Preloader({ onComplete }: { onComplete: () => void }) {
  const [phase, setPhase] = useState<'enter' | 'settle' | 'exit'>('enter');

  useEffect(() => {
    let done = false;

    const settleTimer = setTimeout(() => {
      if (!done) setPhase('settle');
    }, 800);

    const exitTimer = setTimeout(() => {
      if (!done) setPhase('exit');
    }, 1600);

    const finish = () => {
      if (!done) {
        done = true;
        onComplete();
      }
    };

    // Hard fallback in case window.load already fired or never does.
    const doneTimer = setTimeout(finish, 2200);

    // Fire onComplete as soon as the window (and its assets) have loaded,
    // but no earlier than the exit animation start so the visual completes.
    const start = performance.now();
    const minDelay = 1600;
    const onLoad = () => {
      const elapsed = performance.now() - start;
      const remaining = minDelay - elapsed;
      if (remaining > 0) {
        setTimeout(finish, remaining);
      } else {
        finish();
      }
    };

    if (document.readyState === 'complete') {
      onLoad();
    } else {
      window.addEventListener('load', onLoad, { once: true });
    }

    return () => {
      done = true;
      clearTimeout(settleTimer);
      clearTimeout(exitTimer);
      clearTimeout(doneTimer);
      window.removeEventListener('load', onLoad);
    };
  }, [onComplete]);

  const progressWidth = phase === 'settle' || phase === 'exit' ? '100%' : '0%';

  return (
    <AnimatePresence>
      {phase !== 'exit' && (
        <motion.div
          className="fixed inset-0 z-[9999] flex items-center justify-center bg-background"
          initial={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.5, ease: [0.4, 0, 0.2, 1] }}
        >
          <div className="relative flex flex-col items-center gap-8">
            {/* Name reveal */}
            <div className="flex items-center gap-1 overflow-hidden">
              {'Avneesh'.split('').map((char, i) => (
                <motion.span
                  key={i}
                  className="text-4xl md:text-6xl font-bold tracking-tight text-gradient"
                  initial={{ y: 80, opacity: 0, rotateX: -90 }}
                  animate={{ y: 0, opacity: 1, rotateX: 0 }}
                  transition={{
                    duration: 0.5,
                    delay: 0.1 + i * 0.05,
                    ease: [0.4, 0, 0.2, 1],
                  }}
                >
                  {char}
                </motion.span>
              ))}
            </div>

            {/* Progress bar */}
            <div className="w-48 h-[2px] bg-surface-light rounded-full overflow-hidden">
              <motion.div
                className="h-full bg-accent"
                initial={{ width: '0%' }}
                animate={{ width: progressWidth }}
                transition={{ duration: 0.8, ease: [0.4, 0, 0.2, 1] }}
              />
            </div>

            {/* Subtitle */}
            <motion.p
              className="text-sm text-text-secondary font-mono-accent tracking-widest uppercase"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.4, duration: 0.4 }}
            >
              Loading
            </motion.p>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
