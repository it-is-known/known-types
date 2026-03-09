# This is free and unencumbered software released into the public domain.

module OpenAPI; end
module OpenAPI::Transforms; end

##
# Hoist `nullable` properties from referenced schemas into the referencing schema.
module OpenAPI::Transforms::HoistNullables
  ##
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schemas!(schemas)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    schemas.each_value do |schema|
      transform_schema!(schema, schemas)
    end
  end # transform_schemas!

  protected

  ##
  # @param  [Hash] A JSON Schema schema type
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schema!(schema, schemas)
    raise ArgumentError, schema.inspect unless schema.is_a?(Hash)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    case
      when schema[:'$ref']
        ref = schema[:'$ref'].split('/').last.to_sym
        #p [ref, schema[:nullable], schemas[ref][:nullable]]
        schema[:nullable] = true if schemas[ref][:nullable]
      when schema[:oneOf], schema[:anyOf]
        variants = schema[:oneOf] || schema[:anyOf] || []
        variants.each do |variant|
          transform_schema!(variant, schemas)
        end
      when schema[:allOf]
        variants = schema[:allOf] || []
        if variants.size == 2 && variants.any? { it[:'$ref'] } && variants.any? { it[:nullable] }
          variants.each { schema.merge!(it) }
          schema.delete(:allOf)
        else
          variants.each { transform_schema!(it, schemas) }
        end
      when schema[:properties] || schema[:type] == 'object'
        properties = schema[:properties] || {}
        properties.each_value do |property|
          transform_schema!(property, schemas)
        end
      when schema[:items] || schema[:type] == 'array'
        transform_schema!(schema[:items], schemas) if schema[:items]
      else # nothing to do
    end
  end # transform_schema!
end # OpenAPI::Transforms::HoistNullables
