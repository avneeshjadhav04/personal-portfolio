import { motion } from 'framer-motion';
import { Briefcase, Calendar, Award } from 'lucide-react';
import TiltCard from './TiltCard';
import SectionGlow from './SectionGlow';

const experiences = [
  {
    role: 'Technical Team Member',
    company: 'Association of Computer Engineering Students',
    period: '2023 - 2024',
    description:
      'Active member of the university technical society, contributing to the technical growth of the student community through workshops, mentoring, and event organization.',
    skills: ['Workshops', 'Mentoring', 'Event Organizing', 'Content Curation'],
    highlights: [
      'Conducted workshops on technical skill improvement',
      'Mentored juniors in programming and AI/ML',
      'Organized upcoming technical events and hackathons',
      'Managed content curation for society platforms',
    ],
  },
];

const achievements = [
  {
    title: 'Research Paper Selection',
    desc: 'Paper selected for the 11th ICTIS conference in Bangkok, Thailand',
    icon: Award,
    image: '/photo-conference.jpg',
  },
  {
    title: 'HackerRank Golden Badges',
    desc: 'Golden badges in Python, C++, Java, and Problem Solving',
    icon: Award,
    image: '/hackerrank.png',
  },
  {
    title: 'LeetCode Milestone',
    desc: '279 problems solved demonstrating strong DSA skills',
    icon: Award,
    image: '/leetcode.png',
  },
];

const container = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: { staggerChildren: 0.2 },
  },
};

const itemVariant = {
  hidden: { opacity: 0, x: -40 },
  show: { opacity: 1, x: 0, transition: { duration: 0.7 } },
};

const cardVariant = {
  hidden: { opacity: 0, y: 40, scale: 0.95 },
  show: { opacity: 1, y: 0, scale: 1, transition: { duration: 0.6 } },
};

export default function Experience() {
  return (
    <section id="experience" className="pt-32 pb-24 md:pt-40 md:pb-32 px-6 relative overflow-hidden">
      <SectionGlow color="#6366F1" position="top-left" size="lg" opacity={0.3} />
      <div className="max-w-4xl mx-auto relative z-10">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7 }}
          className="text-center mb-16"
        >
          <h2 className="text-sm font-semibold text-accent-teal uppercase tracking-[0.2em] mb-4 font-mono-accent">
            Journey
          </h2>
          <h3 className="text-3xl md:text-5xl font-bold mb-6 tracking-tight">Experience & Achievements</h3>
          <p className="text-text-secondary max-w-2xl mx-auto leading-relaxed">
            From academic leadership to competitive programming milestones — 
            a track record of consistent growth and impact.
          </p>
        </motion.div>

        {/* Experience Timeline */}
        <motion.div
          variants={container}
          initial="hidden"
          whileInView="show"
          viewport={{ once: true, margin: '-50px' }}
          className="relative border-l border-border ml-4 md:ml-0 md:pl-0 mb-20"
        >
          {experiences.map((exp) => (
            <motion.div
              key={exp.role + exp.company}
              variants={itemVariant}
              className="mb-12 md:mb-16 last:mb-0 relative pl-8 md:pl-12"
            >
              <div className="absolute left-[-5px] md:left-[-5px] top-2 w-3 h-3 rounded-full bg-accent-teal shadow-[0_0_15px_rgba(20,184,166,0.6)]" />

              <div className="flex flex-col md:flex-row md:items-center gap-2 md:gap-4 mb-3">
                <div className="flex items-center gap-3">
                  <div className="p-2 rounded-lg bg-surface-light border border-border">
                    <Briefcase size={18} className="text-accent-teal" />
                  </div>
                  <h4 className="text-xl font-bold text-text-primary">{exp.role}</h4>
                </div>
                <span className="hidden md:block text-border">|</span>
                <span className="text-accent-indigo font-medium">{exp.company}</span>
              </div>

              <div className="flex items-center gap-2 text-sm text-text-secondary mb-4 ml-[3px] font-mono-accent">
                <Calendar size={14} />
                {exp.period}
              </div>

              <p className="text-text-secondary leading-relaxed mb-4 ml-[3px]">
                {exp.description}
              </p>

              <ul className="space-y-2 ml-[3px] mb-4">
                {exp.highlights.map((highlight) => (
                  <li key={highlight} className="flex items-start gap-2 text-sm text-text-secondary">
                    <span className="mt-1.5 w-1.5 h-1.5 rounded-full bg-accent-teal shrink-0 shadow-[0_0_5px_rgba(20,184,166,0.5)]" />
                    {highlight}
                  </li>
                ))}
              </ul>

              <div className="flex flex-wrap gap-2 ml-[3px]">
                {exp.skills.map((skill) => (
                  <span
                    key={skill}
                    className="px-3 py-1 text-xs font-medium rounded-full bg-accent-teal/10 text-accent-teal border border-accent-teal/20 font-mono-accent"
                  >
                    {skill}
                  </span>
                ))}
              </div>
            </motion.div>
          ))}
        </motion.div>

        {/* Achievements Grid */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7 }}
          className="text-center mb-10"
        >
          <h3 className="text-2xl md:text-3xl font-bold mb-2">Key Achievements</h3>
          <p className="text-text-secondary">Milestones that define my journey</p>
        </motion.div>

        <motion.div
          variants={container}
          initial="hidden"
          whileInView="show"
          viewport={{ once: true }}
          className="grid md:grid-cols-3 gap-6"
        >
          {achievements.map((ach) => (
            <motion.div key={ach.title} variants={cardVariant}>
              <TiltCard className="h-full">
                <div className="group h-full rounded-2xl glass-card overflow-hidden cursor-pointer">
                  <div className="aspect-[16/10] overflow-hidden bg-surface-light">
                    <img
                      src={ach.image}
                      alt={ach.title}
                      className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
                    />
                  </div>
                  <div className="p-6">
                    <div className="inline-flex p-3 rounded-xl bg-accent-teal/10 border border-accent-teal/20 mb-4">
                      <ach.icon size={24} className="text-accent-teal" />
                    </div>
                    <h4 className="font-bold text-text-primary mb-2">{ach.title}</h4>
                    <p className="text-sm text-text-secondary">{ach.desc}</p>
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
