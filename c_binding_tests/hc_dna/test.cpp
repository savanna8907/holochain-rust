#include "test.h"
#include "../../dna_c_binding/include/dna_c_binding.h"

#include <stdio.h>

void TestHcDna::serializeAndDeserialize() {
  Dna *dna;
  Dna *dna2;
  char *buf;

  dna = holochain_dna_create();
  buf = holochain_dna_to_json(dna);
  holochain_dna_free(dna);

  dna2 = holochain_dna_create_from_json(buf);
  holochain_dna_string_free(buf);

  buf = holochain_dna_get_dna_spec_version(dna2);

  QCOMPARE(QString("2.0"), QString(buf));

  holochain_dna_string_free(buf);
  holochain_dna_free(dna2);
}

void TestHcDna::canGetName() {
  Dna *dna = holochain_dna_create_from_json("{\"name\":\"test\"}");
  char *buf = holochain_dna_get_name(dna);

  QCOMPARE(QString("test"), QString(buf));

  holochain_dna_string_free(buf);
  holochain_dna_free(dna);
}

void TestHcDna::canSetName() {
  Dna *dna = holochain_dna_create();

  holochain_dna_set_name(dna, "test");

  char *buf = holochain_dna_get_name(dna);

  QCOMPARE(QString("test"), QString(buf));

  holochain_dna_string_free(buf);
  holochain_dna_free(dna);
}

QTEST_MAIN(TestHcDna)
