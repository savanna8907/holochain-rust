#ifndef HOLOCHAIN_DNA_C_BINDING_H
#define HOLOCHAIN_DNA_C_BINDING_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void Dna;
extern Dna *holochain_dna_create();
extern Dna *holochain_dna_create_from_json(const char *buf);
extern void holochain_dna_free(Dna *ptr);
extern char *holochain_dna_to_json(const Dna *ptr);
extern void holochain_dna_string_free(char *s);

extern char *holochain_dna_get_name(const Dna *ptr);
extern void holochain_dna_set_name(const Dna *ptr, const char *val);

extern char *holochain_dna_get_description(const Dna *ptr);
extern void holochain_dna_set_description(const Dna *ptr, const char *val);

extern char *holochain_dna_get_version(const Dna *ptr);
extern void holochain_dna_set_version(const Dna *ptr, const char *val);

extern char *holochain_dna_get_uuid(const Dna *ptr);
extern void holochain_dna_set_uuid(const Dna *ptr, const char *val);

extern char *holochain_dna_get_dna_spec_version(const Dna *ptr);
extern void holochain_dna_set_dna_spec_version(const Dna *ptr, const char *val);

#ifdef __cplusplus
}
#endif

#endif
