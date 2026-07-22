import { motion } from 'framer-motion';
import { Bot, Stethoscope, Phone, Calendar, Globe, Home, CreditCard, Cpu } from 'lucide-react';
import TiltCard from './TiltCard';
import SectionGlow from './SectionGlow';

const projects = [
  {
    title: 'Autonomous Lead Generation System',
    description:
      'A fully automated client outreach and lead generation system that identifies prospects, personalizes outreach, and manages follow-ups using AI agents.',
    tags: ['Automation', 'Python', 'APIs', 'n8n'],
    icon: Bot,
    year: '2026',
  },
  {
    title: 'Voice AI Call Handler',
    description:
      'A fully automated inbound call handling system using voice AI, capable of understanding caller intent, answering queries, and routing calls intelligently.',
    tags: ['Voice AI', 'VAPI', 'APIs', 'n8n', 'Prompt Engineering'],
    icon: Phone,
    year: 'Feb 2025',
  },
  {
    title: 'AI Lead Qualification Agent',
    description:
      'An intelligent appointment setter and lead qualification agent that engages prospects, scores leads, and books meetings autonomously.',
    tags: ['AI Agents', 'Automation', 'APIs', 'n8n'],
    icon: Calendar,
    year: 'Jun 2025',
  },
  {
    title: 'LinkedIn Content Automation',
    description:
      'An automated system that generates, schedules, and posts engaging LinkedIn content using AI-driven copywriting and trend analysis.',
    tags: ['Automation', 'Content AI', 'APIs', 'n8n'],
    icon: Globe,
    year: 'Jul 2025',
  },
  {
    title: 'Housing Price Predictor',
    description:
      'A Linear Regression-based model that predicts housing prices from structured data features, demonstrating core ML fundamentals with clean data pipelines.',
    tags: ['Linear Regression', 'Python', 'scikit-learn', 'Pandas'],
    icon: Home,
    year: 'Feb 2024',
  },
  {
    title: 'Credit Card Fraud Detection',
    description:
      'A Binary Classification model that detects fraudulent credit card transactions with high precision, using feature engineering and ensemble techniques.',
    tags: ['Binary Classification', 'Python', 'ML', 'Pandas'],
    icon: CreditCard,
    year: 'Jan 2024',
  },
  // Featured projects already shown above; listed last in All Projects for completeness
  {
    title: 'Project Vulcan: AI Assistant',
    description:
      'An open-source platform that helps you operate AI with terminal-level access for your daily workflows, making them simpler, more secure, and self-hosted.',
    tags: ['TypeScript', 'React', 'Rust', 'Axum', 'SQLite', 'AI'],
    icon: Cpu,
    year: '2026',
  },
  {
    title: 'Kovero AI: AI Powered Healthcare Claims Assistance Platform',
    description:
      'A full-fledged user-centric insurance claims assistance platform that simplifies finding the right health policies and helps users prepare for claims. Integrates AI using OCR and transformer-based LLMs for document verification and query resolution.',
    tags: ['Next.js', 'React', 'TypeScript', 'Rust', 'Axum', 'SQLite', 'Docker', 'AI'],
    icon: Stethoscope,
    year: '2026',
  },
  {
    title: 'LLM From Scratch',
    description:
      '124M parameter language model trained from scratch on 2B tokens. Built every layer in PyTorch, no Trainer.train(). Validation perplexity 14.8, trained in 5 hours. Live API, weights, and code available.',
    tags: ['PyTorch', 'LLM', 'Transformers', 'AI', 'NLP'],
    icon: Cpu,
    year: '2026',
  },
];

const container = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: { staggerChildren: 0.1 },
  },
};

const cardVariant = {
  hidden: { opacity: 0, y: 50, scale: 0.95 },
  show: { opacity: 1, y: 0, scale: 1, transition: { duration: 0.6 } },
};

export default function Projects() {
  return (
    <section id="all-projects" className="pt-32 pb-12 md:pt-40 md:pb-20 px-6 relative overflow-hidden">
      <SectionGlow color="#8B5CF6" position="top-left" size="lg" opacity={0.3} />
      <div className="max-w-6xl mx-auto relative z-10">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7 }}
          className="text-center mb-16"
        >
          <h2 className="text-3xl md:text-5xl font-bold mb-6 tracking-tight">All Projects</h2>
          <p className="text-text-secondary max-w-2xl mx-auto leading-relaxed">
            A showcase of hands-on work across <span className="font-mono-accent text-accent-teal">AI automation</span>, 
            machine learning, and intelligent systems.
          </p>
        </motion.div>

        <motion.div
          variants={container}
          initial="hidden"
          whileInView="show"
          viewport={{ once: true, margin: '-80px' }}
          className="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
        >
          {projects.map((project) => (
            <motion.div key={project.title} variants={cardVariant}>
              <TiltCard className="h-full">
                <div className="group h-full p-6 rounded-2xl glass-card cursor-pointer">
                  <div className="flex items-start justify-between mb-5">
                    <div className="p-3 rounded-xl bg-accent-teal/10 border border-accent-teal/20">
                      <project.icon size={24} className="text-accent-teal" />
                    </div>
                    <span className="text-xs font-medium text-text-secondary bg-background/60 px-3 py-1 rounded-full border border-border font-mono-accent">
                      {project.year}
                    </span>
                  </div>

                  <h4 className="text-xl font-bold text-text-primary group-hover:text-gradient transition-all duration-300 mb-3">
                    {project.title}
                  </h4>

                  <p className="text-sm text-text-secondary leading-relaxed mb-6">
                    {project.description}
                  </p>

                  <div className="flex flex-wrap gap-2">
                    {project.tags.map((tag) => (
                      <span
                        key={tag}
                        className="px-3 py-1 text-xs font-medium rounded-full bg-background/60 border border-border text-text-secondary font-mono-accent"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </motion.div>
      </div>
    </section>
  );
}
